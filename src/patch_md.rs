//! PATCH.md parsing and serialization

use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::fmt::Write;

use crate::models::{FilePatch, PatchDocument, PatchMetadata};

/// Parse a PATCH.md file into a PatchDocument
pub fn parse(content: &str) -> Result<PatchDocument> {
    let lines: Vec<&str> = content.lines().collect();
    let mut doc = PatchDocument::default();
    let mut in_metadata = false;
    let mut in_patch = false;
    let mut current_file = String::new();
    let mut current_diff_lines: Vec<String> = Vec::new();
    let mut metadata_lines: Vec<String> = Vec::new();

    let header_regex = Regex::new(r"^##\s+(\w+)")?;
    let file_header_regex = Regex::new(r"^###\s+(.+)$")?;
    let diff_start_regex = Regex::new(r"^```diff$")?;
    let code_end_regex = Regex::new(r"^```$")?;

    for (_i, line) in lines.iter().enumerate() {
        // Check for section headers
        if let Some(caps) = header_regex.captures(line) {
            let section = caps.get(1).unwrap().as_str();

            if section == "Metadata" {
                in_metadata = true;
                in_patch = false;
                metadata_lines.clear();
            } else if section == "Patches" {
                in_metadata = false;
                in_patch = true;
            }
            continue;
        }

        // Check for file headers within patches section
        if let Some(caps) = file_header_regex.captures(line) {
            // Save previous patch if exists
            if !current_file.is_empty() && !current_diff_lines.is_empty() {
                doc.add_patch(FilePatch::new(
                    &current_file,
                    current_diff_lines.join("\n"),
                ));
            }

            current_file = caps.get(1).unwrap().as_str().to_string();
            current_diff_lines.clear();
            in_patch = true;
            continue;
        }

        // Check for diff code block start
        if diff_start_regex.is_match(line) {
            continue;
        }

        // Check for code block end
        if code_end_regex.is_match(line) {
            // Save current patch
            if !current_file.is_empty() && !current_diff_lines.is_empty() {
                doc.add_patch(FilePatch::new(
                    &current_file,
                    current_diff_lines.join("\n"),
                ));
            }
            current_file.clear();
            current_diff_lines.clear();
            continue;
        }

        // Collect content
        if in_metadata {
            // Handle table format: | key | value |
            if line.starts_with('|') && !line.contains("Key") && !line.contains("---") {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    let key = parts[1].trim();
                    let value = parts[2].trim();
                    if !key.is_empty() && !value.is_empty() {
                        metadata_lines.push(format!("{}: {}", key, value));
                    }
                }
            } else if !line.trim().is_empty() && !line.contains("---") {
                metadata_lines.push(line.to_string());
            }
        } else if in_patch && !current_file.is_empty() {
            current_diff_lines.push(line.to_string());
        }
    }

    // Parse metadata
    doc.metadata = parse_metadata(&metadata_lines)?;

    Ok(doc)
}

/// Parse metadata section
fn parse_metadata(lines: &[String]) -> Result<PatchMetadata> {
    let mut metadata = PatchMetadata::default();

    for line in lines {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            match key.as_str() {
                "version" => metadata.version = value.to_string(),
                "target" => metadata.target = value.to_string(),
                "created" => {
                    metadata.created = DateTime::parse_from_rfc3339(value)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                }
                "author" => metadata.author = Some(value.to_string()),
                "description" => metadata.description = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Ok(metadata)
}

/// Serialize a PatchDocument back to PATCH.md format
pub fn serialize(doc: &PatchDocument) -> Result<String> {
    let mut output = String::new();

    // Header
    writeln!(output, "# PATCH.md")?;
    writeln!(output)?;

    // Metadata section
    writeln!(output, "## Metadata")?;
    writeln!(output)?;
    writeln!(output, "| Key | Value |")?;
    writeln!(output, "|-----|-------|")?;
    writeln!(output, "| version | {} |", doc.metadata.version)?;
    writeln!(output, "| target | {} |", doc.metadata.target)?;
    writeln!(
        output,
        "| created | {} |",
        doc.metadata.created.to_rfc3339()
    )?;

    if let Some(ref author) = doc.metadata.author {
        writeln!(output, "| author | {} |", author)?;
    }

    if let Some(ref desc) = doc.metadata.description {
        writeln!(output, "| description | {} |", desc)?;
    }

    writeln!(output)?;

    // Patches section
    writeln!(output, "## Patches")?;
    writeln!(output)?;

    for patch in &doc.patches {
        writeln!(output, "### {}", patch.path)?;
        writeln!(output)?;
        writeln!(output, "```diff")?;
        writeln!(output, "{}", patch.diff)?;
        writeln!(output, "```")?;
        writeln!(output)?;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_serialize_roundtrip() {
        let content = r#"# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | upstream@v1.0.0 |
| created | 2026-04-24T12:00:00+00:00 |
| author | test |
| description | Test patch |

## Patches

### src/main.rs

```diff
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("Hello");
 }
```
"#;

        let doc = parse(content).unwrap();
        assert_eq!(doc.metadata.version, "1.0");
        assert_eq!(doc.metadata.target, "upstream@v1.0.0");
        assert_eq!(doc.patches.len(), 1);
        assert_eq!(doc.patches[0].path, "src/main.rs");

        let serialized = serialize(&doc).unwrap();
        assert!(serialized.contains("version | 1.0"));
        assert!(serialized.contains("src/main.rs"));
    }
}
