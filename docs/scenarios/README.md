# Real-World Scenarios

This directory contains practical examples of how `patch-md` can be used to maintain customizations across upstream updates.

## Scenario Index

| # | Scenario | Use Case | Files Typically Modified |
|---|----------|----------|-------------------------|
| 01 | [Config File Customization](./01-config-file-customization.md) | TOML, YAML, INI configurations | `config.toml`, `settings.yaml` |
| 02 | [CSS Theme Customization](./02-css-theme-customization.md) | UI library theming | `themes/*.css`, `styles/*.scss` |
| 03 | [Environment-Specific Settings](./03-environment-settings.md) | Dev/staging/prod configs | `.env`, `config/*.env` |
| 04 | [Multi-File Coordinated Changes](./04-multi-file-changes.md) | Library upgrades with API calls | `src/**/*.rs`, `lib/**/*.js` |
| 05 | [Library Version Pinning](./05-library-version-pinning.md) | Dependency management | `package.json`, `requirements.txt`, `Cargo.toml` |

## Quick Reference

### Common Workflows

```bash
# 1. Initialize for your fork
patch-md init --target "upstream@version" --author "your-name"

# 2. After making changes, capture them
patch-md add <file> --original .original/<file>

# 3. When upstream updates
patch-md status        # See what changed
patch-md diff <file>   # View specific patch
patch-md apply --force # Apply your patches

# 4. Remove a patch if no longer needed
patch-md remove <file>
```

### File Organization

```
project/
├── PATCH.md           # Your patch manifest (commit this!)
├── .original/         # Original upstream files
│   ├── config.toml
│   ├── package.json
│   └── src/
│       └── main.rs
├── config.toml        # Your customized version
├── package.json       # Your customized version
└── src/
    └── main.rs        # Your customized version
```

## Choosing the Right Scenario

- **Single config file**: Start with [Scenario 01](./01-config-file-customization.md)
- **UI/styling changes**: See [Scenario 02](./02-css-theme-customization.md)
- **Multiple environments**: [Scenario 03](./03-environment-settings.md) is your guide
- **Code + config changes**: [Scenario 04](./04-multi-file-changes.md) shows coordinated updates
- **Dependency management**: [Scenario 05](./05-library-version-pinning.md) handles version files

## Tips

1. **Always keep `.original/` directory** - this is your reference for generating diffs
2. **Commit PATCH.md to version control** - it documents your customizations
3. **Run `patch-md status` regularly** - to know when upstream changed
4. **Use `--dry-run`** - to preview changes before applying
5. **Use `--force`** - when you're confident and want to override conflicts