# Scenario 5: Library Version Pinning

## Problem Statement

You're using a library that has breaking changes between versions. You need to pin specific versions across multiple configuration files (package.json, requirements.txt, Cargo.toml) and ensure your version constraints are preserved when the upstream project updates its dependency specifications.

## Project Structure

```
project/
├── package.json          # Node.js dependencies
├── requirements.txt      # Python dependencies
├── Cargo.toml           # Rust dependencies
├── PATCH.md
└── .original/
```

## Setup

```bash
mkdir -p scenario-5 .original

# Original package.json
cat > package.json << 'EOF'
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21",
    "axios": "^1.6.0"
  }
}
EOF

# Original requirements.txt
cat > requirements.txt << 'EOF'
django==4.2.0
requests==2.31.0
celery==5.3.0
EOF

# Original Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = "1.35"
serde = "1.0"
reqwest = "0.11"
config = "0.13"
EOF

# Save originals
cp package.json requirements.txt Cargo.toml .original/
```

## Your Version Pinning Strategy

You need to pin specific versions due to:
- Security vulnerabilities in certain versions
- API compatibility requirements
- Performance optimizations in specific versions

### Your Versions

```bash
cat > package.json << 'EOF'
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "4.18.2",     # Pinned: security fix in this version
    "lodash": "4.17.21",     # Pinned: stable API
    "axios": "1.6.8",        # Pinned: performance improvements
    "helmet": "7.1.0"        # Added: security middleware
  }
}
EOF

cat > requirements.txt << 'EOF'
django==4.2.10           # Pinned: security patches
requests==2.31.0         # Pinned: stable
celery==5.3.4            # Pinned: bug fixes
redis==5.0.1             # Added: new dependency
EOF

cat > Cargo.toml << 'EOF'
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = "1.35.1"          # Pinned: latest stable
serde = { version = "1.0", features = ["derive"] }  # Added derive feature
serde_json = "1.0"       # Added: JSON support
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }  # Enhanced
config = "0.13.3"         # Pinned: bug fixes
tracing = "0.1"           # Added: logging
EOF
```

## Tracking with patch-md

```bash
# Initialize
patch-md init --target "upstream-template@v1.0.0" --author "devops"

# Add all dependency files
patch-md add package.json --original .original/package.json
patch-md add requirements.txt --original .original/requirements.txt
patch-md add Cargo.toml --original .original/Cargo.toml
```

### Generated PATCH.md

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | upstream-template@v1.0.0 |
| created | 2026-04-24T12:00:00Z |
| author | devops |

## Patches

### package.json

```diff
--- a/package.json
+++ b/package.json
@@ -2,6 +2,7 @@
   "name": "my-project",
   "version": "1.0.0",
   "dependencies": {
-    "express": "^4.18.0",
+    "express": "4.18.2",
     "lodash": "^4.17.21",
-    "axios": "^1.6.0"
+    "axios": "1.6.8",
+    "helmet": "7.1.0"
   }
 }
```

### requirements.txt

```diff
--- a/requirements.txt
+++ b/requirements.txt
@@ -1,3 +1,4 @@
-django==4.2.0
+django==4.2.10
 requests==2.31.0
-celery==5.3.0
+celery==5.3.4
+redis==5.0.1
```

### Cargo.toml

```diff
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -5,6 +5,8 @@ edition = "2021"

 [dependencies]
-tokio = "1.35"
+tokio = "1.35.1"
 serde = "1.0"
-reqwest = "0.11"
+serde_json = "1.0"
+reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
 config = "0.13"
+tracing = "0.1"
```

```

## Upstream Updates Dependency Specs

The upstream template has been updated with new dependencies and version ranges.

```bash
cat > .original/package.json << 'EOF'
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.19.0",      # Upstream updated
    "lodash": "^4.17.21",
    "axios": "^1.6.0",
    "cors": "^2.8.5",          # New upstream dependency
    "compression": "^1.7.4"     # New upstream dependency
  },
  "devDependencies": {         # New section
    "jest": "^29.0.0"
  }
}
EOF

cat > .original/requirements.txt << 'EOF'
django==5.0.0              # Major version update
requests==2.32.0           # Updated
celery==5.3.6              # Updated
prometheus-client==0.19.0   # New dependency
EOF

cat > .original/Cargo.toml << 'EOF'
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = "1.36"              # Updated
serde = "1.0"
reqwest = "0.12"           # Major version update
config = "0.14"            # Updated
thiserror = "1.0"          # New dependency
anyhow = "1.0"             # New dependency
EOF
```

## Reconciliation

```bash
# Check status
patch-md status

# Output:
# PATCH.md Status
# ================
# Target: upstream-template@v1.0.0
# Created: 2026-04-24T12:00:00Z
# Patches: 3
#
# [MODIFIED] package.json
# [MODIFIED] requirements.txt
# [MODIFIED] Cargo.toml

# Apply - your pinned versions preserved, new upstream deps added
patch-md apply --force
```

### Results

#### package.json

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "4.18.2",        // YOUR: Pinned version preserved
    "lodash": "^4.17.21",
    "axios": "1.6.8",          // YOUR: Pinned version preserved
    "helmet": "7.1.0",         // YOUR: New dependency preserved
    "cors": "^2.8.5",         // NEW: From upstream
    "compression": "^1.7.4"    // NEW: From upstream
  },
  "devDependencies": {         // NEW: From upstream
    "jest": "^29.0.0"
  }
}
```

#### requirements.txt

```text
django==4.2.10              // YOUR: Pinned version preserved
requests==2.31.0             // YOUR: Pinned version preserved
celery==5.3.4                // YOUR: Pinned version preserved
redis==5.0.1                 // YOUR: New dependency preserved
prometheus-client==0.19.0    // NEW: From upstream
```

#### Cargo.toml

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = "1.35.1"                            // YOUR: Pinned version preserved
serde = { version = "1.0", features = ["derive"] }  // YOUR: Enhanced
serde_json = "1.0"                          // YOUR: New dependency preserved
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }  // YOUR: Enhanced
config = "0.13.3"                           // YOUR: Pinned version preserved
tracing = "0.1"                             // YOUR: New dependency preserved
thiserror = "1.0"                           // NEW: From upstream
anyhow = "1.0"                              // NEW: From upstream
```

## Key Takeaways

- **Version pinning preserved**: Your exact versions maintained
- **New dependencies inherited**: Upstream's new deps (cors, jest, thiserror) added
- **Enhanced configurations kept**: Your feature additions (helmet, derive, rustls-tls) preserved
- **Major version changes rejected**: Upstream's Django 5.0 and reqwest 0.12 not applied

## Advanced: Version Constraint Strategy

For more complex version constraints, consider:

```bash
# For lock files (package-lock.json, Pipfile.lock, Cargo.lock)
# These should be regenerated after reconciliation

# Regenerate Node.js lock file
npm install

# Regenerate Python lock file
pip-compile

# Regenerate Rust lock file
cargo update
```

## Commands Used

```bash
patch-md init --target "upstream-template@v1.0.0" --author "devops"
patch-md add package.json --original .original/package.json
patch-md add requirements.txt --original .original/requirements.txt
patch-md add Cargo.toml --original .original/Cargo.toml
patch-md status
patch-md diff package.json
patch-md apply --force
```