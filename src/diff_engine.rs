//! Diff generation engine using the `similar` crate

use std::fmt::Write;

use anyhow::Result;
use regex::Regex;
use similar::{ChangeTag, TextDiff};

use crate::models::DiffHunk;

/// Generate a unified diff between two strings
pub fn generate_diff(original: &str, modified: &str, file_path: &str) -> Result<String> {
    let diff = TextDiff::from_lines(original, modified);
    let mut output = String::new();

    // Header
    writeln!(output, "--- a/{}", file_path)?;
    writeln!(output, "+++ b/{}", file_path)?;

    // Group changes into hunks
    let mut current_hunk_lines: Vec<String> = Vec::new();
    let mut original_line: usize = 1;
    let mut modified_line: usize = 1;
    let mut hunk_original_start = 0;
    let mut hunk_modified_start = 0;
    let mut changes_in_hunk = 0;
    let mut context_before: Vec<String> = Vec::new();

    let context_size = 3;

    for change in diff.iter_all_changes() {
        let line = change.value().to_string();
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };

        match change.tag() {
            ChangeTag::Equal => {
                if changes_in_hunk > 0 {
                    // Continue collecting context after changes
                    current_hunk_lines.push(format!(" {}", line));
                    original_line += 1;
                    modified_line += 1;
                    changes_in_hunk += 1;
                } else {
                    // Accumulate context before
                    context_before.push(line.clone());
                    if context_before.len() > context_size {
                        context_before.remove(0);
                    }
                    original_line += 1;
                    modified_line += 1;
                }
            }
            ChangeTag::Delete => {
                if changes_in_hunk == 0 {
                    // Start a new hunk
                    hunk_original_start = original_line.saturating_sub(context_before.len());
                    hunk_modified_start = modified_line.saturating_sub(context_before.len());

                    // Add context before
                    current_hunk_lines.extend(context_before.iter().map(|l| format!(" {}", l)));
                    original_line += context_before.len();
                    modified_line += context_before.len();
                    context_before.clear();
                }
                current_hunk_lines.push(format!("-{}", line));
                original_line += 1;
                changes_in_hunk += 1;
            }
            ChangeTag::Insert => {
                if changes_in_hunk == 0 {
                    // Start a new hunk
                    hunk_original_start = original_line.saturating_sub(context_before.len());
                    hunk_modified_start = modified_line.saturating_sub(context_before.len());

                    // Add context before
                    current_hunk_lines.extend(context_before.iter().map(|l| format!(" {}", l)));
                    original_line += context_before.len();
                    modified_line += context_before.len();
                    context_before.clear();
                }
                current_hunk_lines.push(format!("+{}", line));
                modified_line += 1;
                changes_in_hunk += 1;
            }
        }

        // Flush hunk if we have enough context after changes
        if changes_in_hunk > 0 {
            let remaining_context = collect_remaining_context(&diff, original_line, modified_line);
            current_hunk_lines.extend(remaining_context.iter().map(|l| format!(" {}", l)));

            if should_flush_hunk(&current_hunk_lines, context_size) {
                flush_hunk(
                    &mut output,
                    &current_hunk_lines,
                    hunk_original_start,
                    hunk_modified_start,
                )?;
                current_hunk_lines.clear();
                changes_in_hunk = 0;
                context_before.clear();
            }
        }
    }

    // Flush remaining hunk
    if !current_hunk_lines.is_empty() {
        flush_hunk(
            &mut output,
            &current_hunk_lines,
            hunk_original_start,
            hunk_modified_start,
        )?;
    }

    Ok(output)
}

fn collect_remaining_context(_diff: &TextDiff<str>, _orig_line: usize, _mod_line: usize) -> Vec<String> {
    // In a real implementation, we'd collect the actual remaining lines
    // For simplicity, we return empty here and rely on the main loop
    Vec::new()
}

fn should_flush_hunk(lines: &[String], context_size: usize) -> bool {
    // Count context lines at the end
    let mut trailing_context = 0;
    for line in lines.iter().rev() {
        if line.starts_with(' ') {
            trailing_context += 1;
        } else {
            break;
        }
    }
    trailing_context >= context_size
}

fn flush_hunk(
    output: &mut String,
    lines: &[String],
    orig_start: usize,
    mod_start: usize,
) -> Result<()> {
    let mut orig_count = 0;
    let mut mod_count = 0;

    for line in lines {
        match line.chars().next() {
            Some('-') => orig_count += 1,
            Some('+') => mod_count += 1,
            Some(' ') | None => {
                orig_count += 1;
                mod_count += 1;
            }
            _ => {}
        }
    }

    writeln!(output, "@@ -{},{} +{},{} @@", orig_start, orig_count, mod_start, mod_count)?;
    for line in lines {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

/// Parse a unified diff into hunks
pub fn parse_diff(diff_content: &str) -> Result<Vec<DiffHunk>> {
    let lines: Vec<&str> = diff_content.lines().collect();
    let mut hunks = Vec::new();

    let hunk_header_regex = Regex::new(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")?;

    let mut current_hunk: Option<DiffHunk> = None;

    for line in lines {
        if let Some(caps) = hunk_header_regex.captures(line) {
            // Save previous hunk
            if let Some(h) = current_hunk.take() {
                hunks.push(h);
            }

            let orig_start: usize = caps.get(1).unwrap().as_str().parse().unwrap_or(1);
            let orig_count: usize = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
            let mod_start: usize = caps.get(3).unwrap().as_str().parse().unwrap_or(1);
            let mod_count: usize = caps.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);

            current_hunk = Some(DiffHunk {
                original_start: orig_start,
                original_count: orig_count,
                new_start: mod_start,
                new_count: mod_count,
                lines: Vec::new(),
            });
        } else if line.starts_with('-') {
            if let Some(ref mut hunk) = current_hunk {
                hunk.lines
                    .push(crate::models::DiffLine::Deletion(line[1..].to_string()));
            }
        } else if line.starts_with('+') {
            if let Some(ref mut hunk) = current_hunk {
                hunk.lines
                    .push(crate::models::DiffLine::Addition(line[1..].to_string()));
            }
        } else if line.starts_with(' ') || (!line.starts_with('\\') && !line.is_empty()) {
            if let Some(ref mut hunk) = current_hunk {
                let content = if line.starts_with(' ') {
                    line[1..].to_string()
                } else {
                    line.to_string()
                };
                hunk.lines.push(crate::models::DiffLine::Context(content));
            }
        }
    }

    // Don't forget the last hunk
    if let Some(h) = current_hunk {
        hunks.push(h);
    }

    Ok(hunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_diff() {
        let original = "line1\nline2\nline3\n";
        let modified = "line1\nmodified\nline3\nline4\n";

        let diff = generate_diff(original, modified, "test.txt").unwrap();
        assert!(diff.contains("--- a/test.txt"));
        assert!(diff.contains("+++ b/test.txt"));
        assert!(diff.contains("@@"));
    }
}
