# Real-World Testing Journeys

This directory contains documented test journeys of patch-md with real open-source projects.

## Journeys

| # | Journey | Description | Status |
|---|---------|-------------|--------|
| 01 | [Real-World Testing](./01-real-world-testing.md) | Testing with fd, GitHub CLI, rust-cookbook | ✅ Complete |
| 02 | [AI Resolve Real World Test](./02-ai-resolve-real-world-test.md) | Testing Claude Code integration with fd | ✅ Complete |

## Journey 01: Basic Functionality Testing

Tests basic patch-md operations:
- `init` - Initialize PATCH.md
- `add` - Capture changes with intents
- `diff` - View recorded patches
- `status` - Check patch compatibility
- `apply` - Reapply patches after revert

## Journey 02: AI-Powered Resolution

Tests the core AI-powered conflict resolution feature:
- Real merge conflict scenario with sharkdp/fd
- Claude Code integration via CLI
- Intent-based resolution
- Smart combination of upstream + user changes

### Key Result

Claude Code successfully:
1. Understood the user's intent from PATCH.md
2. Identified non-conflicting changes
3. Combined MAX_RESULTS (upstream) + LOG_LEVEL (user)
4. Produced valid, compilable Rust code

## Running Tests

```bash
# Test with fd
cd /tmp && git clone --depth 1 https://github.com/sharkdp/fd.git fd-test
cd fd-test
patch-md init --target "sharkdp/fd@v9.0" --author "tester"
# ... make customizations ...

# Simulate conflict
# ... edit files to create conflict markers ...

# Resolve with AI
patch-md resolve --file src/config.rs
```
