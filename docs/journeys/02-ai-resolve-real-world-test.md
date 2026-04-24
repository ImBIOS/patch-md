# AI Agent Resolve - Real World Test

This document chronicles the real-world test of the AI-powered conflict resolution feature using Claude Code with an actual open-source project.

## Project Tested

- **Repository**: [sharkdp/fd](https://github.com/sharkdp/fd) - A fast alternative to `find`
- **Language**: Rust
- **Test Date**: 2026-04-24

## Test Scenario

### 1. Initial Setup

We started with a basic `src/config.rs` file:

```rust
use std::env;

pub struct Config {
    pub debug: bool,
    pub color: String,
    pub max_depth: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            color: "auto".to_string(),
            max_depth: None,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            debug: env::var("FD_DEBUG").is_ok(),
            color: env::var("FD_COLOR").unwrap_or_else(|_| "auto".to_string()),
            max_depth: env::var("FD_MAX_DEPTH")
                .ok()
                .and_then(|v| v.parse().ok()),
        }
    }
}
```

### 2. User Customization with Intent

Added a customization with clear intent:

```bash
patch-md add src/config.rs --original .original/config.rs \
  --intent "Add verbose logging configuration constant to enable configurable logging levels (0=none, 1=error, 2=warn, 3=info, 4=debug) for better debugging experience"
```

This created `PATCH.md` with:

```markdown
### src/config.rs

> **Intent**: Add verbose logging configuration constant to enable configurable logging levels (0=none, 1=error, 2=warn, 3=info, 4=debug) for better debugging experience

```diff
--- a/src/config.rs
+++ b/src/config.rs
@@ -20,6 +20,9 @@
 }

 

+/// Verbose logging level (0=none, 1=error, 2=warn, 3=info, 4=debug)
+pub const LOG_LEVEL: u8 = 3;
+
 impl Config {
```

### 3. Simulated Upstream Change

Simulated an upstream change that adds a different constant in the same location:

```rust
/// Maximum number of search results to return (0 = unlimited)
pub const MAX_RESULTS: usize = 0;
```

### 4. Merge Conflict Created

When trying to apply patches, a conflict was created:

```rust
<<<<<<< HEAD
/// Maximum number of search results to return (0 = unlimited)
pub const MAX_RESULTS: usize = 0;

=======
/// Verbose logging level (0=none, 1=error, 2=warn, 3=info, 4=debug)
pub const LOG_LEVEL: u8 = 3;

>>>>>>> patch-md
```

## The AI Resolution

### Prompt Sent to Claude Code

```markdown
You are helping resolve git-style merge conflicts in: src/config.rs

## Conflicted File Content:
[full file with conflict markers]

## User's PATCH.md (their customizations with intent):
[full PATCH.md content including intent]

## Your Task:
1. Analyze the conflict markers (<<<<<<< HEAD, =======, >>>>>>> patch-md)
2. Understand what changes each side represents
3. Use the PATCH.md intent to determine the correct resolution
4. Combine BOTH changes (the upstream change AND the user's customization) when they don't conflict
5. Preserve the user's intended functionality

## IMPORTANT - Output Format:
Output ONLY the complete resolved file content wrapped in a Rust code block (```rust ... ```).
Do NOT include any explanations or markdown outside the code block.
The code block content will be extracted and used directly to replace the file.
```

### Claude Code's Resolution

Claude Code correctly identified:
- HEAD contains: `MAX_RESULTS` constant
- patch-md contains: `LOG_LEVEL` constant (with clear intent to add logging)

Since these are **non-conflicting additions**, Claude Code intelligently combined both:

```rust
/// Maximum number of search results to return (0 = unlimited)
pub const MAX_RESULTS: usize = 0;

/// Verbose logging level (0=none, 1=error, 2=warn, 3=info, 4=debug)
pub const LOG_LEVEL: u8 = 3;
```

## Results

| Aspect | Result |
|--------|--------|
| Conflict Detection | ✅ Detected correctly |
| Intent Understanding | ✅ Understood user's logging intent |
| Smart Combination | ✅ Preserved both changes |
| Output Format | ✅ Valid Rust code |

## Key Observations

1. **Intent-Based Resolution**: Claude Code used the PATCH.md intent to understand that the user's `LOG_LEVEL` constant was for debugging purposes, not just a random addition.

2. **Non-Conflicting Merge**: The AI correctly identified that adding `MAX_RESULTS` and `LOG_LEVEL` in the same location doesn't create a logical conflict.

3. **Preserved User Customization**: The user's intended functionality (configurable logging levels) was preserved despite upstream changes.

## Conclusion

The AI-powered resolve command successfully implements Theo's vision:
- ✅ Reviews PATCH.md to understand user's original goal
- ✅ Intelligently combines upstream and user changes
- ✅ Re-implements user's intended functionality in new codebase

This demonstrates the power of combining:
- **Dual-record keeping** (diff + intent in PATCH.md)
- **AI understanding** (Claude Code interprets intent)
- **Automated resolution** (no manual intervention needed)
