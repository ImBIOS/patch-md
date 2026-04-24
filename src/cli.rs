//! CLI argument parsing and command routing

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
    Add {
        /// Path to the modified file
        file: String,

        /// Path to the original file (for comparison)
        #[arg(short, long)]
        original: Option<String>,
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

    /// Reconcile patches with upstream changes
    Reconcile {
        /// Path to the upstream directory
        #[arg(short, long)]
        upstream: String,
    },

    /// Remove a patch from PATCH.md
    Remove {
        /// Path to the file to remove patch for
        file: String,
    },
}
