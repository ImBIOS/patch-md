# PATCH.md Specification

A standard format for encoding user customizations to software, enabling self-healing and reconciliation when upstream receives updates.

## Format Overview

PATCH.md is a markdown file that contains metadata and unified diffs for tracking user modifications to source files.

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
2. Contains a fenced code block with `diff` language tag
3. Uses unified diff format (standard diff -u output)
4. Sections are separated by blank lines

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
2. **Add**: Capture changes to a file as a patch
3. **Apply**: Apply patches to current upstream
4. **Diff**: Show differences from original
5. **Status**: Check compatibility with current upstream
6. **Reconcile**: Attempt automatic merge of conflicts

### Patch Application Strategy

1. Read PATCH.md and extract patches
2. For each patched file:
   - Load original version (from git or backup)
   - Load user's patched version
   - Load current upstream version
   - Perform 3-way merge: `original + user_patch → current`
3. Handle conflicts with conflict markers
4. Write merged result

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

## Examples

See `examples/` directory for sample PATCH.md files.
