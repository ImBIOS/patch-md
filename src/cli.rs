//! CLI argument parsing and command routing
//!
//! Implements Theo's vision:
//! - Intent-Based Customization via --intent flag
//! - Agent-Assisted Reconciliation via resolve command

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "patch-md",
    about = "Self-healing patch system for software customization",
    long_about = None,
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new PATCH.md file
    Init {
        /// The upstream target (e.g., "upstream@v1.0.0")
        #[arg(short, long)]
        target: Option<String>,

        /// Author name
        #[arg(short, long)]
        author: Option<String>,
    },

    /// Add a file's current state as a patch
    ///
    /// Use --intent to describe WHY this change was made.
    /// This implements dual-record keeping: storing both the diff and the intent.
    Add {
        /// Path to the modified file
        file: String,

        /// Path to the original file (for comparison)
        #[arg(short, long)]
        original: Option<String>,

        /// Intent-Based Customization: describe the purpose of this change
        /// Example: "Enable debug mode for development"
        #[arg(short = 'i', long)]
        intent: Option<String>,
    },

    /// Apply patches to current files
    Apply {
        /// Show what would be done without making changes
        #[arg(short, long)]
        dry_run: bool,

        /// Force apply even if there are conflicts
        #[arg(short, long)]
        force: bool,
    },

    /// Show diff between PATCH.md and current state
    Diff {
        /// Show diff for a specific file only
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Check status of patches
    Status,

    /// Reconcile patches with upstream changes (basic 3-way merge)
    Reconcile {
        /// Path to the upstream directory
        #[arg(short, long)]
        upstream: String,
    },

    /// Agent-Assisted Reconciliation: Use AI to resolve conflicts
    ///
    /// This implements Theo's vision: "If a conflict arises, an AI agent
    /// reviews the patch.md file, understands the user's original goal,
    /// and attempts to re-implement that functionality in the new codebase."
    Resolve {
        /// Path to the file with conflicts
        #[arg(short, long)]
        file: Option<String>,

        /// Use Claude Code for AI-powered resolution (default)
        #[arg(short, long, default_value = "claude")]
        agent: Option<String>,
    },

    /// Remove a patch from PATCH.md
    Remove {
        /// Path to the file to remove patch for
        file: String,
    },
}
