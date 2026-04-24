# patch-md Documentation

Welcome to the patch-md documentation. patch-md is a CLI tool that implements the PATCH.md standard for self-healing software customization.

## Quick Start

```bash
# Install from source
cargo install --path .

# Initialize for your project
patch-md init --target "upstream@v1.0.0" --author "your-name"

# Make your customizations...

# Capture changes as patches
patch-md add config.toml --original .original/config.toml

# Check status
patch-md status

# Apply patches after upstream update
patch-md apply --force
```

## Documentation Structure

```
docs/
├── README.md           # This file
├── scenarios/          # Real-world usage scenarios
│   ├── README.md      # Scenario overview
│   ├── 01-config-file-customization.md
│   ├── 02-css-theme-customization.md
│   ├── 03-environment-settings.md
│   ├── 04-multi-file-changes.md
│   └── 05-library-version-pinning.md
└── CONTRIBUTING.md     # How to contribute
```

## Core Concepts

### What is PATCH.md?

PATCH.md is a markdown file format that stores your customizations as unified diffs. It allows you to:

1. **Track** changes to files across updates
2. **Reconcile** your changes with upstream modifications
3. **Self-heal** when dependencies update

### The Workflow

```
┌─────────────────────────────────────────────────────────┐
│  1. Fork upstream                                       │
│  2. Make your customizations                            │
│  3. Run `patch-md add <file>` to capture changes        │
│  4. When upstream updates:                             │
│     - Pull new upstream version                        │
│     - Run `patch-md apply` to reconcile                 │
│  5. Your customizations preserved!                     │
└─────────────────────────────────────────────────────────┘
```

## Scenarios

| Scenario | Description |
|----------|-------------|
| [Config File Customization](./scenarios/01-config-file-customization.md) | Preserve database and server settings |
| [CSS Theme Customization](./scenarios/02-css-theme-customization.md) | Keep your brand colors across updates |
| [Environment Settings](./scenarios/03-environment-settings.md) | Manage dev/staging/prod configs |
| [Multi-File Changes](./scenarios/04-multi-file-changes.md) | Coordinate changes across the codebase |
| [Library Version Pinning](./scenarios/05-library-version-pinning.md) | Maintain dependency version constraints |

## Commands

| Command | Description |
|---------|-------------|
| `init` | Create new PATCH.md |
| `add` | Add a file as a patch |
| `apply` | Apply patches to files |
| `diff` | Show recorded patches |
| `status` | Check patch status |
| `reconcile` | 3-way merge with upstream |
| `remove` | Remove a patch |

For detailed command options, run `patch-md <command> --help`.

## Resources

- [GitHub Repository](https://github.com/ImBIOS/patch-md)
- [PATCH.md Specification](../SPEC.md)
- [Quick Start Guide](scenarios/README.md)