# PATCH.md Specification

A standard format for encoding user customizations to software, enabling self-healing and reconciliation when upstream receives updates.

## Format Overview

PATCH.md is a markdown file that contains metadata, unified diffs, and **intent descriptions** for tracking user modifications to source files.

## Key Features

- **Dual-Record Keeping**: Records both the actual code edit AND the descriptive intent
- **Intent-Based Customization**: Describes WHY each change was made
- **Agent-Assisted Reconciliation**: AI agent (Claude Code) helps resolve conflicts

## File Structure

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | upstream@v1.2.3 |
| created | 2026-04-24T12:00:00Z |
| author | username |
| description | Custom configuration changes |

## Patches

### filename.ext

> **Intent**: Enable debug mode for development environment

```diff
--- a/filename.ext
+++ b/filename.ext
@@ -line,count +line,count @@
-old content
+new content
```

### another-file.txt
...
```

## Metadata Fields

| Field | Required | Description |
|-------|----------|-------------|
| version | Yes | PATCH.md format version (currently 1.0) |
| target | Yes | Upstream version this patch targets (format: repo@version) |
| created | Yes | ISO 8601 timestamp of patch creation |
| author | No | Author identifier |
| description | No | Human-readable description of changes |

## Patch Sections

Each patch section:
1. Starts with `### <filename>` header
2. Optionally contains `> **Intent**: <description>` line (dual-record keeping)
3. Contains a fenced code block with `diff` language tag
4. Uses unified diff format (standard diff -u output)
5. Sections are separated by blank lines

### Intent Field

The intent field is a key part of **Intent-Based Customization**:

```markdown
### src/config.rs

> **Intent**: Enable debug logging for development environment
```

This allows AI agents to understand WHY a change was made, enabling smart reconciliation when upstream changes.

## Diff Format

Uses standard unified diff format:
- `---` line: original file path
- `+++` line: modified file path
- `@@ -start,count +start,count @@`: hunk header
- `-` prefix: removed line
- `+` prefix: added line
- ` ` prefix: context line (unchanged)

## Implementation Requirements

### Core Features

1. **Init**: Create new PATCH.md scaffold
2. **Add**: Capture changes to a file as a patch (with optional --intent)
3. **Apply**: Apply patches to current upstream
4. **Diff**: Show differences from original (including intent)
5. **Status**: Check compatibility with current upstream
6. **Reconcile**: Attempt automatic merge of conflicts
7. **Resolve**: Agent-assisted AI reconciliation (Claude Code)

### CLI Commands

| Command | Description |
|---------|-------------|
| `patch-md init` | Initialize new PATCH.md |
| `patch-md add <file> --intent "description"` | Add patch with intent |
| `patch-md apply` | Apply all patches |
| `patch-md diff` | Show patches and intents |
| `patch-md status` | Check patch compatibility |
| `patch-md reconcile --upstream <path>` | 3-way merge with upstream |
| `patch-md resolve` | AI-assisted conflict resolution |

### Patch Application Strategy

1. Read PATCH.md and extract patches (with intents)
2. For each patched file:
   - Load original version (from git or backup)
   - Load user's patched version
   - Load current upstream version
   - Perform 3-way merge: `original + user_patch → current`
3. Handle conflicts with conflict markers
4. Use AI agent (Claude Code) for smart resolution if needed

### Conflict Resolution

When patches overlap:
1. **Auto-merge**: If changes are in different locations, merge automatically
2. **Conflict**: If changes touch same lines, insert conflict markers:
   ```diff
   <<<<<<< ours
   user content
   =======
   upstream content
   >>>>>>> theirs
   ```
3. **AI Resolution**: Use `patch-md resolve` to have Claude Code intelligently resolve based on the user's intent

### Agent-Assisted Reconciliation (AI Resolution)

The `resolve` command implements Theo's vision:

```
1. User runs: patch-md resolve
2. System reads PATCH.md to understand user's customizations and intents
3. System finds all files with conflict markers
4. Claude Code is invoked with:
   - The conflicted file content
   - The user's PATCH.md (including intents)
   - Instructions to resolve based on user's original goal
5. Claude Code outputs resolved file content
6. System writes the resolution
```

This enables "an AI agent reviews the patch.md file, understands the user's original goal, and attempts to re-implement that functionality in the new codebase."

## Examples

See `docs/` directory for real-world scenario documentation.