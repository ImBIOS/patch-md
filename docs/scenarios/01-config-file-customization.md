# Scenario 1: Configuration File Customization

## Problem Statement

You use an open-source project that reads configuration from `config.toml`. The maintainer adds new default settings in an update, and you need to keep your custom values while accepting the new structure.

## Setup

```bash
# Create scenario directory
mkdir -p scenario-1 && cd scenario-1
mkdir -p .original

# Original config from upstream
cat > config.toml << 'EOF'
[database]
host = "localhost"
port = 5432
name = "app_db"

[server]
port = 3000
debug = false

[features]
beta_features = false
EOF

# Save original
cp config.toml .original/
```

## Your Customization

You need to change the configuration for your production environment:

```bash
# Modify config.toml
cat > config.toml << 'EOF'
[database]
host = "prod-db.example.com"
port = 5432
name = "production_db"

[server]
port = 8080
debug = false

[features]
beta_features = true
EOF
```

## Using patch-md

```bash
# Initialize PATCH.md
patch-md init --target "myapp@v1.0.0" --author "devops"

# Capture your changes
patch-md add config.toml --original .original/config.toml
```

### Generated PATCH.md

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | myapp@v1.0.0 |
| created | 2026-04-24T12:00:00Z |
| author | devops |

## Patches

### config.toml

```diff
--- a/config.toml
+++ b/config.toml
@@ -1,11 +1,11 @@
 [database]
-host = "localhost"
+host = "prod-db.example.com"
-port = 5432
-name = "app_db"
+name = "production_db"

 [server]
-port = 3000
+port = 8080

 [features]
-beta_features = false
+beta_features = true
```

```

## Simulating Upstream Update

```bash
# Maintainer adds new settings
cat > config.toml << 'EOF'
[database]
host = "localhost"
port = 5432
name = "app_db"

[server]
port = 3000
debug = false

[features]
beta_features = false
legacy_mode = false          # NEW
performance_monitoring = true  # NEW

[security]                     # NEW SECTION
ssl_enabled = true
EOF
```

## Reconciliation

```bash
# Check status
patch-md status

# Output:
# PATCH.md Status
# ================
# Target: myapp@v1.0.0
# Created: 2026-04-24T12:00:00Z
# Patches: 1
#
# [MODIFIED] config.toml

# Apply your patches to the updated upstream
patch-md apply --force

# Result - your custom values preserved, new settings added
cat config.toml
```

### Final config.toml after reconciliation

```toml
[database]
host = "prod-db.example.com"
port = 5432
name = "production_db"

[server]
port = 8080
debug = false

[features]
beta_features = true
legacy_mode = false
performance_monitoring = true

[security]
ssl_enabled = true
```

## Key Takeaways

- Your database host, name, and port customization preserved
- Server port customization preserved
- Beta features toggle preserved
- New upstream settings (`legacy_mode`, `performance_monitoring`, `security`) added automatically

## Commands Used

```bash
patch-md init --target "myapp@v1.0.0" --author "devops"
patch-md add config.toml --original .original/config.toml
patch-md status
patch-md apply --force
```