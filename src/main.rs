//! patch-md: Self-healing patch system for software customization
//!
//! PATCH.md is a standard for encoding user customizations into a markdown file
//! that allows for self-healing and easy reconciliation when upstream receives updates.

mod cli;
mod commands;
mod diff_engine;
mod merge_engine;
mod models;
mod patch_md;

use anyhow::Result;
use cli::Cli;
use clap::Parser;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init { target, author } => {
            commands::init(target.as_deref(), author.as_deref())?;
        }
        cli::Commands::Add { file, original } => {
            commands::add(&file, original.as_deref())?;
        }
        cli::Commands::Apply { dry_run, force } => {
            commands::apply(dry_run, force)?;
        }
        cli::Commands::Diff { file } => {
            commands::diff(file.as_deref())?;
        }
        cli::Commands::Status => {
            commands::status()?;
        }
        cli::Commands::Reconcile { upstream } => {
            commands::reconcile(&upstream)?;
        }
        cli::Commands::Remove { file } => {
            commands::remove(&file)?;
        }
    }

    Ok(())
}
