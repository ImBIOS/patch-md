//! Three-way merge engine for applying patches

use anyhow::{bail, Result};
use similar::{ChangeTag, TextDiff};

use crate::diff_engine::parse_diff;
use crate::models::{ConflictRegion, MergeResult};

/// Apply a patch (in unified diff format) to original content
/// Returns the patched content or an error
pub fn apply_patch(original: &str, patch: &str) -> Result<MergeResult> {
    let hunks = parse_diff(patch)?;
    let mut result = original.to_string();
    let mut offset: isize = 0;

    for hunk in hunks {
        let hunk_start = hunk.original_start as isize + offset - 1;
        let hunk_end = hunk_start + hunk.original_count as isize;

        if hunk_start < 0 || hunk_end > result.lines().count() as isize {
            bail!("Hunk out of bounds");
        }

        let start = hunk_start as usize;
        let end = hunk_end as usize;

        // Find the lines to replace
        let mut new_lines: Vec<String> = Vec::new();

        for line in &hunk.lines {
            match line {
                crate::models::DiffLine::Context(c) => new_lines.push(c.clone()),
                crate::models::DiffLine::Addition(a) => new_lines.push(a.clone()),
                crate::models::DiffLine::Deletion(_) => {} // Skip deletions, they're replaced
            }
        }

        // Replace the hunk in result
        let lines: Vec<String> = result.lines().map(|s| s.to_string()).collect();
        let mut new_result: Vec<String> = Vec::new();

        new_result.extend(lines[..start].iter().cloned());
        new_result.extend(new_lines);
        new_result.extend(lines[end..].iter().cloned());

        result = new_result.join("\n");

        // Update offset based on what changed
        let deletions = hunk
            .lines
            .iter()
            .filter(|l| matches!(l, crate::models::DiffLine::Deletion(_)))
            .count();
        let additions = hunk
            .lines
            .iter()
            .filter(|l| matches!(l, crate::models::DiffLine::Addition(_)))
            .count();
        offset += additions as isize - deletions as isize;
    }

    Ok(MergeResult::Success(result))
}

/// Three-way merge: combine changes from original -> modified with original -> upstream
/// This is used when the user has patches and wants to apply them to a new upstream version
pub fn three_way_merge(
    original: &str,
    modified: &str,
    upstream: &str,
) -> Result<MergeResult> {
    let diff = TextDiff::from_lines(original, upstream);

    let mut conflicts: Vec<ConflictRegion> = Vec::new();
    let mut result = String::new();
    let mut _orig_line = 1;
    let mut _up_line = 1;
    let mut mod_line = 1;
    let mut in_conflict = false;
    let mut conflict_ours = String::new();
    let mut conflict_theirs = String::new();
    let mut conflict_start = 0;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                if in_conflict {
                    // End conflict
                    conflicts.push(ConflictRegion {
                        start_line: conflict_start,
                        end_line: result.lines().count(),
                        ours: conflict_ours.clone(),
                        theirs: conflict_theirs.clone(),
                    });
                    conflict_ours.clear();
                    conflict_theirs.clear();
                    in_conflict = false;
                }
                result.push_str(change.value());
                _orig_line += 1;
                _up_line += 1;
                mod_line += 1;
            }
            ChangeTag::Delete => {
                // Check if modified has the same change
                let mod_lines: Vec<&str> = modified.lines().collect();
                let mod_has_change = mod_line <= mod_lines.len()
                    && mod_lines[mod_line - 1] != change.value().trim_end_matches('\n');

                if mod_has_change {
                    if !in_conflict {
                        conflict_start = result.lines().count();
                        in_conflict = true;
                        conflict_ours.push_str(change.value());
                        conflict_theirs.push_str(change.value());
                    }

                    // Both changed differently - conflict
                    conflict_ours.push_str(change.value());
                } else {
                    // Only upstream changed, apply to result
                    if !in_conflict {
                        result.push_str(change.value());
                    } else {
                        conflict_theirs.push_str(change.value());
                    }
                }
                _orig_line += 1;
                _up_line += 1;
                mod_line += 1;
            }
            ChangeTag::Insert => {
                if in_conflict {
                    conflict_theirs.push_str(change.value());
                } else {
                    result.push_str(change.value());
                }
                _up_line += 1;
            }
        }
    }

    if in_conflict {
        conflicts.push(ConflictRegion {
            start_line: conflict_start,
            end_line: result.lines().count(),
            ours: conflict_ours,
            theirs: conflict_theirs,
        });
    }

    // Also apply any additions from modified that aren't in upstream
    let mod_diff = TextDiff::from_lines(original, modified);
    for change in mod_diff.iter_all_changes() {
        if change.tag() == ChangeTag::Insert && !result.contains(change.value()) {
            // This is a user addition that upstream doesn't have
            // Check if it's already in result
            if !result.lines().any(|l| l.trim() == change.value().trim()) {
                result.push_str(change.value());
            }
        }
    }

    if conflicts.is_empty() {
        Ok(MergeResult::Success(result))
    } else {
        Ok(MergeResult::Conflict {
            content: result,
            conflicts,
        })
    }
}

/// Format conflict markers in the content
pub fn format_with_conflict_markers(
    content: &str,
    conflicts: &[ConflictRegion],
) -> String {
    // For simplicity, return the content as-is with conflict markers
    // A full implementation would insert proper conflict markers
    let mut result = content.to_string();

    for conflict in conflicts.iter().rev() {
        let lines: Vec<&str> = result.lines().collect();
        if conflict.start_line < lines.len() {
            let mut new_content = String::new();

            // Add lines before conflict
            for (i, line) in lines.iter().enumerate() {
                if i == conflict.start_line {
                    new_content.push_str("<<<<<<< ours\n");
                    new_content.push_str(&conflict.ours);
                    new_content.push('\n');
                    new_content.push_str("=======\n");
                    new_content.push_str(&conflict.theirs);
                    new_content.push('\n');
                    new_content.push_str(">>>>>>> theirs\n");
                }
                new_content.push_str(line);
                new_content.push('\n');
            }

            result = new_content;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_simple_patch() {
        let original = "line1\nline2\nline3\n";
        let patch = r#"--- a/test.txt
+++ b/test.txt
@@ -1,3 +1,4 @@
 line1
+new line
 line2
 line3
"#;

        let result = apply_patch(original, patch).unwrap();
        match result {
            MergeResult::Success(content) => {
                assert!(content.contains("new line"));
            }
            MergeResult::Conflict { .. } => panic!("Unexpected conflict"),
        }
    }
}
