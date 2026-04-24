# Patch.md - Self-Healing Software

A CLI tool for the PATCH.md standard - enabling self-healing and easy reconciliation when upstream software receives updates. Features **Intent-Based Customization** and **Agent-Assisted Reconciliation** powered by Claude Code.

## Overview

PATCH.md is a way for users to encode their specific customizations into a markdown file that allows for self-healing and easy reconciliation when the core software receives updates from the maintainer. This enables a future of **"self-forking" software**.

### Key Features

- **Dual-Record Keeping**: Records both the actual code edit AND the descriptive intent
- **Intent-Based Customization**: Describes WHY each change was made
- **Agent-Assisted Reconciliation**: Claude Code AI helps resolve conflicts intelligently

## Installation

```bash
# From source
cargo install --path .

# Or build manually
cargo build --release
```

## Quick Start

```bash
# Initialize PATCH.md for your project
patch-md init --target "upstream@v1.0.0"

# Make your customizations to files...

# Capture your changes as patches (with intent!)
patch-md add src/config.rs --original .original/src/config.rs \
    --intent "Enable debug logging for development environment"

# Check status
patch-md status

# View diff (includes your intent)
patch-md diff

# When upstream updates, reconcile
patch-md reconcile --upstream /path/to/upstream

# Use AI to resolve conflicts intelligently
patch-md resolve
```

## Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize a new PATCH.md file |
| `add <file>` | Add a file's current state as a patch |
| `apply` | Apply patches to current files |
| `diff` | Show diff between PATCH.md and current state |
| `status` | Check status of patches |
| `reconcile` | Reconcile patches with upstream changes |
| `resolve` | AI-assisted conflict resolution (Claude Code) |
| `remove <file>` | Remove a patch from PATCH.md |

## PATCH.md Format

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | upstream@v1.0.0 |
| created | 2026-04-24T12:00:00Z |
| author | username |

## Patches

### src/config.rs

> **Intent**: Enable debug logging for development environment

```diff
--- a/src/config.rs
+++ b/src/config.rs
@@ -5,7 +5,7 @@
-const DEBUG = false;
+const DEBUG = true;
```

```

## How It Works

1. **Capture Original**: Keep original versions of upstream files in `.original/`
2. **Make Changes**: Customize the software to your needs
3. **Record Patches**: Use `patch-md add --intent "..."` to capture changes with intent
4. **Update Upstream**: Pull new changes from upstream
5. **Reconcile**: Use `patch-md reconcile` to merge your changes with upstream
6. **AI Resolution**: Use `patch-md resolve` to have Claude Code intelligently resolve conflicts based on your original intent

## Options

### init
```bash
patch-md init [OPTIONS]

Options:
  --target <target>    Upstream target version (e.g., "repo@v1.0.0")
  --author <author>    Author name
```

### add
```bash
patch-md add <file> [OPTIONS]

Options:
  --original <path>    Path to original file for comparison
  --intent <text>      Intent-Based Customization: describe WHY this change was made
```

### reconcile
```bash
patch-md reconcile --upstream <path>
```

### resolve (AI-Assisted)
```bash
patch-md resolve [OPTIONS]

Options:
  --file <path>    Specific file to resolve (default: all files with conflicts)
```

## AI-Assisted Reconciliation

When `patch-md reconcile` detects conflicts, use `patch-md resolve` to have Claude Code:

1. Read your PATCH.md (with intents)
2. Analyze the conflicted file
3. Understand your original goal
4. Re-implement your functionality in the new codebase

This implements Theo's vision: *"If a conflict arises, an AI agent reviews the patch.md file, understands the user's original goal, and attempts to re-implement that functionality in the new codebase."*

## License

MIT