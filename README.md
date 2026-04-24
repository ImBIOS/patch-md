# Patch.md - Self-Healing Software

A CLI tool for the PATCH.md standard - enabling self-healing and easy reconciliation when upstream software receives updates.

## Overview

PATCH.md is a way for users to encode their specific customizations into a markdown file that allows for self-healing and easy reconciliation when the core software receives updates from the maintainer. This enables a future of **"self-forking" software**.

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

# Capture your changes as patches
patch-md add src/config.rs --original .original/src/config.rs
patch-md add src/main.rs --original .original/src/main.rs

# Check status
patch-md status

# View diff
patch-md diff

# Apply patches (after updating from upstream)
patch-md apply
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
3. **Record Patches**: Use `patch-md add` to capture your changes
4. **Update Upstream**: Pull new changes from upstream
5. **Reconcile**: Use `patch-md reconcile` to merge your changes with upstream

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
```

### apply
```bash
patch-md apply [OPTIONS]

Options:
  --dry-run    Show what would be done without making changes
  --force      Force apply even if there are conflicts
```

## License

MIT
