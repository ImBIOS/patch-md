//! Data models for PATCH.md format
//!
//! Implements Theo's vision for self-healing software:
//! - Dual-Record Keeping: Records both the actual code edit AND the descriptive intent

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata section of PATCH.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMetadata {
    pub version: String,
    pub target: String,
    pub created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Default for PatchMetadata {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            target: "upstream@0.0.0".to_string(),
            created: Utc::now(),
            author: None,
            description: None,
        }
    }
}

/// A single patch for one file
/// Implements dual-record keeping: stores both the diff AND the user's intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePatch {
    /// The file path this patch applies to
    pub path: String,

    /// Intent-Based Customization: describes WHY this change was made
    /// This is the "descriptive logic" that Theo envisioned
    /// Example: "Enable debug mode for development environment"
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,

    /// The unified diff content (the actual code edit)
    pub diff: String,
}

impl FilePatch {
    pub fn new(path: impl Into<String>, diff: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            diff: diff.into(),
            intent: None,
        }
    }

    pub fn with_intent(path: impl Into<String>, diff: impl Into<String>, intent: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            diff: diff.into(),
            intent: Some(intent.into()),
        }
    }
}

/// Complete PATCH.md document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDocument {
    pub metadata: PatchMetadata,
    pub patches: Vec<FilePatch>,
}

impl Default for PatchDocument {
    fn default() -> Self {
        Self {
            metadata: PatchMetadata::default(),
            patches: Vec::new(),
        }
    }
}

impl PatchDocument {
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            metadata: PatchMetadata {
                target: target.into(),
                ..Default::default()
            },
            patches: Vec::new(),
        }
    }

    pub fn add_patch(&mut self, patch: FilePatch) {
        self.patches.push(patch);
    }

    pub fn remove_patch(&mut self, path: &str) -> Option<FilePatch> {
        let idx = self.patches.iter().position(|p| p.path == path)?;
        Some(self.patches.remove(idx))
    }

    pub fn get_patch(&self, path: &str) -> Option<&FilePatch> {
        self.patches.iter().find(|p| p.path == path)
    }
}

/// Represents a single hunk in a diff
#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub original_start: usize,
    pub original_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff
#[derive(Debug, Clone, PartialEq)]
pub enum DiffLine {
    Context(String),
    Addition(String),
    Deletion(String),
}

impl DiffLine {
    pub fn content(&self) -> &str {
        match self {
            DiffLine::Context(s) | DiffLine::Addition(s) | DiffLine::Deletion(s) => s,
        }
    }
}

/// Merge result
#[derive(Debug)]
pub enum MergeResult {
    Success(String),
    Conflict {
        content: String,
        conflicts: Vec<ConflictRegion>,
    },
}

/// Represents a conflict region in merged content
#[derive(Debug, Clone)]
pub struct ConflictRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub ours: String,
    pub theirs: String,
}
