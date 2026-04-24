# patch-md Documentation

Welcome to the patch-md documentation. patch-md is a CLI tool that implements the PATCH.md standard for self-healing software customization with **Intent-Based Customization** and **Agent-Assisted Reconciliation**.

## Quick Start

```bash
# Install from source
cargo install --path .

# Initialize for your project
patch-md init --target "upstream@v1.0.0" --author "your-name"

# Make your customizations...

# Capture changes as patches with intent
patch-md add config.toml --original .original/config.toml \
    --intent "Enable verbose logging for troubleshooting"

# Check status
patch-md status

# When upstream updates, reconcile
patch-md reconcile --upstream /path/to/upstream

# Use AI to resolve conflicts
patch-md resolve
```

## Documentation Structure

```
docs/
├── README.md           # This file
├── claude-code/         # Claude Code integration docs
│   └── headless.md      # Headless mode API reference
```

## Core Concepts

### What is PATCH.md?

PATCH.md is a markdown file format that stores your customizations as unified diffs with **intent descriptions**. It allows you to:

1. **Track** changes to files across updates
2. **Record intent** - WHY each change was made
3. **Reconcile** your changes with upstream modifications
4. **AI-Assisted Resolution** - Claude Code helps resolve conflicts intelligently

### Dual-Record Keeping

PATCH.md implements **Dual-Record Keeping** by storing:

1. **The actual code edit** (the diff)
2. **The descriptive intent** (why the change was made)

Example:
```markdown
### src/config.rs

> **Intent**: Enable debug logging for development environment

```diff
-const DEBUG = false;
+const DEBUG = true;
```
```

### The Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. Fork upstream                                                │
│  2. Make your customizations                                     │
│  3. Run `patch-md add <file> --intent "..."` to capture changes  │
│  4. When upstream updates:                                       │
│     - Pull new upstream version                                 │
│     - Run `patch-md reconcile --upstream <path>`                  │
│  5. If conflicts: run `patch-md resolve` for AI assistance      │
│  6. Your customizations preserved with intent!                  │
└─────────────────────────────────────────────────────────────────┘
```

## Commands

| Command | Description |
|---------|-------------|
| `init` | Create new PATCH.md |
| `add` | Add a file as a patch (with --intent) |
| `apply` | Apply patches to files |
| `diff` | Show recorded patches and intents |
| `status` | Check patch status |
| `reconcile` | 3-way merge with upstream |
| `resolve` | AI-assisted conflict resolution (Claude Code) |
| `remove` | Remove a patch |

For detailed command options, run `patch-md <command> --help`.

## AI-Assisted Reconciliation

When you run `patch-md resolve`, Claude Code:

1. Reads your PATCH.md to understand your customizations and intents
2. Finds all files with conflict markers
3. Analyzes each conflicted file
4. Understands your original goal from the intent descriptions
5. Re-implements your intended functionality in the new codebase

This implements Theo's vision: *"If a conflict arises, an AI agent reviews the patch.md file, understands the user's original goal, and attempts to re-implement that functionality in the new codebase."*

## Resources

- [GitHub Repository](https://github.com/ImBIOS/patch-md)
- [PATCH.md Specification](../SPEC.md)
- [README](../README.md)