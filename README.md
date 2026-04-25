# Patch.md - Self-Healing Software

> **ARCHIVED** — This project has been migrated to [Relay](https://github.com/ImBIOS/relay).
>
> The PATCH.md concept lives on as **Relay Patches** within the `.relay/patches/` directory in the Relay repository. Relay Patches extend the original PATCH.md idea with:
> - Structured spec/QA documentation (`.relay/specs/` + `.relay/qa/`)
> - GitHub Actions workflows for automated fork sync and patch re-application
> - Upstream issue/PR management for forked repos
> - A patch registry (`registry.json`) for tracking all customizations
>
> See [Relay `.relay/README.md`](https://github.com/ImBIOS/relay/tree/main/.relay) for the new documentation system.

---

*"Each time we sync from upstream, an AI agent reviews the patch.md file, understands the user's original goal, and attempts to re-implement that functionality in the new codebase."*

A CLI tool for the PATCH.md standard - enabling self-healing and easy reconciliation when upstream software receives updates. Features **Intent-Based Customization** and **Agent-Assisted Reconciliation** powered by Claude Code.

## Overview

PATCH.md is a way for users to encode their specific customizations into a markdown file that allows for self-healing and easy reconciliation when the core software receives updates from the maintainer. This enables a future of **"self-forking" software**.

### Key Features

- **Dual-Record Keeping**: Records both the actual code edit AND the descriptive intent
- **Intent-Based Customization**: Describes WHY each change was made
- **Agent-Assisted Reconciliation**: Claude Code AI helps resolve conflicts intelligently

## Migration to Relay

The PATCH.md concept has been integrated into the [Relay](https://github.com/ImBIOS/relay) project as part of a broader **Relay Spec, QA & Patch** system:

| PATCH.md Concept | Relay Equivalent |
|------------------|-----------------|
| `PATCH.md` file | `.relay/patches/<name>.md` |
| Intent description | `## Intent` section in patch file |
| Reconciliation notes | `## Reconciliation Notes` section |
| Manual sync | `fork-sync.yml` GitHub workflow |
| CLI tool | Relay's built-in patch management |

## PATCH.md Format

<!--TODO: Need to decide-->

````

## How It Works

<!--TODO: Need to investigate further-->

1. **Capture Original**: Keep original versions of upstream files in `.original/`
2. **Make Changes**: Customize the software to your needs
3. **Record Patches**: Use `patch-md add --intent "..."` to capture changes with intent
4. **Update Upstream**: Pull new changes from upstream
5. **Reconcile**: Use `patch-md reconcile` to merge your changes with upstream
6. **AI Resolution**: Use `patch-md resolve` to have Claude Code intelligently resolve conflicts based on your original intent

## AI-Assisted Reconciliation

Claude Code will analyze the intent behind your changes and the new upstream code to attempt to re-implement your functionality in the updated codebase. The process involves:

1. Read your PATCH.md (with intents)
2. Analyze the conflicted file
3. Understand your original goal
4. Re-implement your functionality in the new codebase
