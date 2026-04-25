# Patch.md - Self-Healing Software

*"Each time we sync from upstream, an AI agent reviews the patch.md file, understands the user's original goal, and attempts to re-implement that functionality in the new codebase."*

A CLI tool for the PATCH.md standard - enabling self-healing and easy reconciliation when upstream software receives updates. Features **Intent-Based Customization** and **Agent-Assisted Reconciliation** powered by Claude Code.

## Overview

PATCH.md is a way for users to encode their specific customizations into a markdown file that allows for self-healing and easy reconciliation when the core software receives updates from the maintainer. This enables a future of **"self-forking" software**.

### Key Features

- **Dual-Record Keeping**: Records both the actual code edit AND the descriptive intent
- **Intent-Based Customization**: Describes WHY each change was made
- **Agent-Assisted Reconciliation**: Claude Code AI helps resolve conflicts intelligently

## PATCH.md Format

````markdown
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
