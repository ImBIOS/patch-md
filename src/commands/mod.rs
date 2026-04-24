//! CLI commands implementation

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use crate::diff_engine::generate_diff;
use crate::merge_engine::{apply_patch, format_with_conflict_markers, three_way_merge};
use crate::models::{FilePatch, MergeResult};
use crate::patch_md::{parse, serialize};

const PATCH_FILENAME: &str = "PATCH.md";

/// Initialize a new PATCH.md file
pub fn init(target: Option<&str>, author: Option<&str>) -> Result<()> {
    let target_str = target.unwrap_or("upstream@0.0.0").to_string();

    let mut doc = crate::models::PatchDocument::new(&target_str);

    if let Some(a) = author {
        doc.metadata.author = Some(a.to_string());
    }

    let content = serialize(&doc)?;
    fs::write(PATCH_FILENAME, content)?;

    println!("Created {} with target: {}", PATCH_FILENAME, target_str);
    Ok(())
}

/// Add a file's current state as a patch
pub fn add(file_path: &str, original_path: Option<&str>, intent: Option<&str>) -> Result<()> {
    let file_path = Path::new(file_path);

    if !file_path.exists() {
        anyhow::bail!("File does not exist: {:?}", file_path);
    }

    let modified_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read modified file: {:?}", file_path))?;

    let original_content = if let Some(orig) = original_path {
        fs::read_to_string(orig)?
    } else {
        // Try to find original in .original/ directory
        let original_path = Path::new(".original").join(file_path);
        if original_path.exists() {
            fs::read_to_string(&original_path)?
        } else {
            // No original, create a simple "new file" diff
            format!("+{}", modified_content.lines().collect::<Vec<_>>().join("\n+"))
        }
    };

    let diff = generate_diff(&original_content, &modified_content, file_path.to_str().unwrap())?;

    let patch = if let Some(i) = intent {
        FilePatch::with_intent(file_path.to_str().unwrap(), diff, i)
    } else {
        FilePatch::new(file_path.to_str().unwrap(), diff)
    };

    // Load or create PATCH.md
    let mut doc = if Path::new(PATCH_FILENAME).exists() {
        let content = fs::read_to_string(PATCH_FILENAME)?;
        parse(&content)?
    } else {
        crate::models::PatchDocument::default()
    };

    // Remove existing patch for this file if any
    doc.remove_patch(file_path.to_str().unwrap());
    doc.add_patch(patch);

    let content = serialize(&doc)?;
    fs::write(PATCH_FILENAME, content)?;

    println!("Added patch for: {}", file_path.display());
    Ok(())
}

/// Apply patches to current files
pub fn apply(dry_run: bool, force: bool) -> Result<()> {
    if !Path::new(PATCH_FILENAME).exists() {
        anyhow::bail!("No {} found. Run 'patch-md init' first.", PATCH_FILENAME);
    }

    let content = fs::read_to_string(PATCH_FILENAME)?;
    let doc = parse(&content)?;

    for patch in &doc.patches {
        let file_path = Path::new(&patch.path);

        if !file_path.exists() {
            println!("Skipping {} (file does not exist)", patch.path);
            continue;
        }

        let current = fs::read_to_string(file_path)?;

        match apply_patch(&current, &patch.diff)? {
            MergeResult::Success(new_content) => {
                if dry_run {
                    println!("Would modify: {}", patch.path);
                    println!("{}", new_content);
                } else {
                    fs::write(file_path, &new_content)?;
                    println!("Applied patch to: {}", patch.path);
                }
            }
            MergeResult::Conflict { content: _, conflicts } => {
                if force {
                    // Apply anyway with conflict markers
                    let marked = format_with_conflict_markers(&current, &conflicts);
                    if dry_run {
                        println!("Would have conflicts in: {}", patch.path);
                    } else {
                        fs::write(file_path, &marked)?;
                        println!("Applied {} with conflict markers", patch.path);
                    }
                } else {
                    println!("Conflicts detected in: {}", patch.path);
                    println!("Run with --force to apply anyway");
                }
            }
        }
    }

    Ok(())
}

/// Show diff between PATCH.md and current state
pub fn diff(file_path: Option<&str>) -> Result<()> {
    if !Path::new(PATCH_FILENAME).exists() {
        anyhow::bail!("No {} found.", PATCH_FILENAME);
    }

    let content = fs::read_to_string(PATCH_FILENAME)?;
    let doc = parse(&content)?;

    if let Some(path) = file_path {
        // Show diff for specific file
        if let Some(patch) = doc.get_patch(path) {
            println!("Patch for {}:", path);
            println!("{}", patch.diff);
            if let Some(ref intent) = patch.intent {
                println!();
                println!("Intent: {}", intent);
            }
        } else {
            println!("No patch found for: {}", path);
        }
    } else {
        // Show all patches
        for patch in &doc.patches {
            println!("=== {} ===", patch.path);
            if let Some(ref intent) = patch.intent {
                println!("Intent: {}", intent);
                println!();
            }
            println!("{}", patch.diff);
            println!();
        }
    }

    Ok(())
}

/// Check status of patches
pub fn status() -> Result<()> {
    if !Path::new(PATCH_FILENAME).exists() {
        println!("No {} found. Run 'patch-md init' first.", PATCH_FILENAME);
        return Ok(());
    }

    let content = fs::read_to_string(PATCH_FILENAME)?;
    let doc = parse(&content)?;

    println!("PATCH.md Status");
    println!("================");
    println!("Target: {}", doc.metadata.target);
    println!("Created: {}", doc.metadata.created.to_rfc3339());
    println!("Patches: {}", doc.patches.len());
    println!();

    for patch in &doc.patches {
        let file_path = Path::new(&patch.path);

        if file_path.exists() {
            let current = fs::read_to_string(file_path)?;
            if current.contains(&patch.diff.replace("+", "").replace("-", "")) {
                println!("[OK] {}", patch.path);
            } else {
                println!("[MODIFIED] {}", patch.path);
            }
        } else {
            println!("[MISSING] {}", patch.path);
        }
    }

    Ok(())
}

/// Reconcile patches with upstream changes
pub fn reconcile(upstream_path: &str) -> Result<()> {
    if !Path::new(PATCH_FILENAME).exists() {
        anyhow::bail!("No {} found.", PATCH_FILENAME);
    }

    let content = fs::read_to_string(PATCH_FILENAME)?;
    let mut doc = parse(&content)?;

    let upstream = Path::new(upstream_path);

    for patch in &mut doc.patches {
        let file_path = Path::new(&patch.path);

        if !file_path.exists() {
            println!("Skipping {} (file does not exist)", patch.path);
            continue;
        }

        if !upstream.exists() {
            println!("Skipping {} (upstream {} does not exist)", patch.path, upstream_path);
            continue;
        }

        let current = fs::read_to_string(file_path)?;
        let _upstream_content = fs::read_to_string(upstream)?;

        // For simplicity, assume upstream has the same file path
        let upstream_file = Path::new(upstream_path).join(file_path);

        if !upstream_file.exists() {
            println!("Skipping {} (no corresponding upstream file)", patch.path);
            continue;
        }

        let upstream_version = fs::read_to_string(&upstream_file)?;

        // Three-way merge
        match three_way_merge(&upstream_version, &current, &upstream_version)? {
            MergeResult::Success(new_content) => {
                fs::write(file_path, &new_content)?;
                println!("Reconciled: {}", patch.path);
            }
            MergeResult::Conflict { content, conflicts } => {
                let marked = format_with_conflict_markers(&content, &conflicts);
                fs::write(file_path, &marked)?;
                println!("Reconciled with conflicts: {}", patch.path);
            }
        }
    }

    Ok(())
}

/// Agent-Assisted Reconciliation: Use AI to resolve conflicts
///
/// This implements Theo's vision: "If a conflict arises, an AI agent
/// reviews the patch.md file, understands the user's original goal,
/// and attempts to re-implement that functionality in the new codebase."
pub fn resolve(file_path: Option<&str>, _agent: Option<&str>) -> Result<()> {
    // Read PATCH.md to understand the user's intended customizations
    let patch_content = if Path::new(PATCH_FILENAME).exists() {
        fs::read_to_string(PATCH_FILENAME)?
    } else {
        anyhow::bail!("No {} found. Run 'patch-md init' first.", PATCH_FILENAME);
    };

    // Determine which files have conflicts
    let files_to_resolve: Vec<String> = if let Some(path) = file_path {
        vec![path.to_string()]
    } else {
        // Find all files with conflict markers
        find_conflicted_files()?
    };

    if files_to_resolve.is_empty() {
        println!("No conflicts found. Run 'patch-md reconcile' first or use --file to specify a file.");
        return Ok(());
    }

    println!("Resolving {} file(s) with Claude Code AI assistance...\n", files_to_resolve.len());

    for file in &files_to_resolve {
        resolve_file_with_claude(file, &patch_content)?;
    }

    println!("\nAll conflicts resolved!");
    Ok(())
}

/// Find all files containing git-style conflict markers
fn find_conflicted_files() -> Result<Vec<String>> {
    let mut files = Vec::new();

    // Search in current directory for files with conflict markers
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains("<<<<<<<") && content.contains("=======") && content.contains(">>>>>>>") {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(files)
}

/// Resolve a single file's conflicts using Claude Code
fn resolve_file_with_claude(file_path: &str, patch_content: &str) -> Result<()> {
    println!("Resolving: {}", file_path);

    // Read the current file content with conflict markers
    let current_content = fs::read_to_string(file_path)?;

    // Build a detailed prompt for Claude Code
    let prompt = build_resolution_prompt(file_path, &current_content, patch_content);

    // Run Claude Code with text output
    let output = Command::new("claude")
        .args([
            "-p",           // Print mode (non-interactive)
            &prompt,
            "--output-format", "text",
            "--no-session-persistence",  // Don't save session
            "--dangerously-skip-permissions",  // Skip permission prompts
        ])
        .output()
        .context("Failed to run Claude Code. Make sure it's installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Claude Code failed: {}", stderr);
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse the response - Claude wraps in code blocks by default
    // We need to extract the content between code block markers
    let resolved_content = extract_code_block(&stdout)
        .unwrap_or_else(|| stdout.trim().to_string());

    if resolved_content.is_empty() {
        println!("  -> Claude Code returned empty response");
        return Ok(());
    }

    // Write the resolved content
    fs::write(file_path, &resolved_content)?;
    println!("  -> Resolved successfully");
    Ok(())
}

/// Extract content from a Claude Code code block response
fn extract_code_block(output: &str) -> Option<String> {
    // Try to find content between ``` and ```
    let lines: Vec<&str> = output.lines().collect();
    
    // Find start marker (```, ```rust, ```python, etc.)
    let start_idx = lines.iter().position(|l| l.starts_with("```") && !l.contains("diff") && !l.contains("json") && !l.contains("mcp"))?;
    
    // Find end marker (```)
    let end_idx = lines[start_idx + 1..].iter().position(|l| *l == "```")?;
    
    // Join lines between start and end
    let content = lines[start_idx + 1..start_idx + 1 + end_idx].join("\n");
    Some(content)
}

/// Build a detailed prompt for Claude Code to resolve conflicts
fn build_resolution_prompt(file_path: &str, conflicted_content: &str, patch_content: &str) -> String {
    format!(
        r#"You are helping resolve git-style merge conflicts in: {}

## Conflicted File Content:
```
{}
```

## User's PATCH.md (their customizations with intent):
```
{}
```

## Your Task:
1. Analyze the conflict markers (<<<<<<< HEAD, =======, >>>>>>> patch-md)
2. Understand what changes each side represents
3. Use the PATCH.md intent to determine the correct resolution
4. Combine BOTH changes (the upstream change AND the user's customization) when they don't conflict
5. Preserve the user's intended functionality

## IMPORTANT - Output Format:
Output ONLY the complete resolved file content wrapped in a Rust code block (```rust ... ```).
Do NOT include any explanations or markdown outside the code block.
The code block content will be extracted and used directly to replace the file."#,
        file_path,
        conflicted_content,
        patch_content
    )
}

/// Remove a patch from PATCH.md
pub fn remove(file_path: &str) -> Result<()> {
    if !Path::new(PATCH_FILENAME).exists() {
        anyhow::bail!("No {} found.", PATCH_FILENAME);
    }

    let content = fs::read_to_string(PATCH_FILENAME)?;
    let mut doc = parse(&content)?;

    if doc.remove_patch(file_path).is_some() {
        let new_content = serialize(&doc)?;
        fs::write(PATCH_FILENAME, new_content)?;
        println!("Removed patch for: {}", file_path);
    } else {
        println!("No patch found for: {}", file_path);
    }

    Ok(())
}