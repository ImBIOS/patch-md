# Scenario 3: Environment-Specific Settings

## Problem Statement

You have an application that needs different configurations for development, staging, and production. You want to track these environment-specific customizations so they persist across upstream updates.

## Project Structure

```
myapp/
├── config/
│   ├── development.env
│   ├── staging.env
│   └── production.env
├── src/
│   └── main.rs
├── PATCH.md
└── .original/
```

## Setup

```bash
mkdir -p scenario-3/config .original

# Original .env.example from upstream
cat > config/.env.example << 'EOF'
# Application
APP_ENV=development
APP_PORT=3000
APP_DEBUG=true

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=myapp_dev

# Cache
CACHE_ENABLED=false
CACHE_TTL=300

# External Services
API_KEY=your_api_key_here
WEBHOOK_URL=https://example.com/webhook

# Feature Flags
ENABLE_ANALYTICS=false
ENABLE_LOGGING=true
EOF

cp config/.env.example .original/
```

## Your Environment Setup

### Development (config/development.env)

```bash
cat > config/development.env << 'EOF'
APP_ENV=development
APP_PORT=3000
APP_DEBUG=true

DB_HOST=localhost
DB_PORT=5432
DB_NAME=myapp_dev

CACHE_ENABLED=false
CACHE_TTL=300

API_KEY=dev_key_12345
WEBHOOK_URL=http://localhost:8080/webhook

ENABLE_ANALYTICS=false
ENABLE_LOGGING=true
EOF
```

### Production (config/production.env)

```bash
cat > config/production.env << 'EOF'
APP_ENV=production
APP_PORT=8080
APP_DEBUG=false

DB_HOST=prod-db.internal
DB_PORT=5432
DB_NAME=myapp_production

CACHE_ENABLED=true
CACHE_TTL=3600

API_KEY=prod_key_SECURE_xyz
WEBHOOK_URL=https://api.example.com/webhook

ENABLE_ANALYTICS=true
ENABLE_LOGGING=true
EOF
```

## Tracking with patch-md

```bash
# Initialize for your fork
patch-md init --target "upstream@v1.5.0" --author "devops-team"

# Capture environment-specific configurations
patch-md add config/development.env --original .original/.env.example
patch-md add config/production.env --original .original/.env.example
```

### Generated PATCH.md

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | upstream@v1.5.0 |
| created | 2026-04-24T12:00:00Z |
| author | devops-team |

## Patches

### config/development.env

```diff
--- a/config/.env.example
+++ b/config/development.env
@@ -3,6 +3,7 @@ APP_DEBUG=true

 # Database
 DB_HOST=localhost
+DB_HOST=localhost  # Already set, but explicit
 DB_PORT=5432
 DB_NAME=myapp_dev

 # External Services
-API_KEY=your_api_key_here
+API_KEY=dev_key_12345
-WEBHOOK_URL=https://example.com/webhook
+WEBHOOK_URL=http://localhost:8080/webhook
```

### config/production.env

```diff
--- a/config/.env.example
+++ b/config/production.env
@@ -1,13 +1,13 @@
 # Application
-APP_ENV=development
+APP_ENV=production
-APP_DEBUG=true
+APP_DEBUG=false

 # Database
-DB_HOST=localhost
+DB_HOST=prod-db.internal
 DB_PORT=5432
-DB_NAME=myapp_dev
+DB_NAME=myapp_production

 # Cache
-CACHE_ENABLED=false
+CACHE_ENABLED=true
-CACHE_TTL=300
+CACHE_TTL=3600

 # External Services
-API_KEY=your_api_key_here
+API_KEY=prod_key_SECURE_xyz
-WEBHOOK_URL=https://example.com/webhook
+WEBHOOK_URL=https://api.example.com/webhook

 # Feature Flags
-ENABLE_ANALYTICS=false
+ENABLE_ANALYTICS=true
```

```

## Upstream Update Adds New Settings

```bash
# Upstream adds new configuration options
cat > .original/.env.example << 'EOF'
# Application
APP_ENV=development
APP_PORT=3000
APP_DEBUG=true

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=myapp_dev

# Cache
CACHE_ENABLED=false
CACHE_TTL=300

# External Services
API_KEY=your_api_key_here
WEBHOOK_URL=https://example.com/webhook

# Feature Flags
ENABLE_ANALYTICS=false
ENABLE_LOGGING=true

# NEW: Security Settings
RATE_LIMIT_ENABLED=true
RATE_LIMIT_REQUESTS=100

# NEW: Monitoring
SENTRY_DSN=
METRICS_ENABLED=false

# NEW: Storage
STORAGE_BACKEND=local
STORAGE_PATH=/data/uploads
EOF
```

## Reconciliation

```bash
# Check status
patch-md status

# Apply patches - your production settings preserved
patch-md apply --force
```

### Result - Production Config Preserved

```env
# Application
APP_ENV=production
APP_PORT=8080
APP_DEBUG=false

# Database
DB_HOST=prod-db.internal
DB_PORT=5432
DB_NAME=myapp_production

# Cache
CACHE_ENABLED=true
CACHE_TTL=3600

# External Services
API_KEY=prod_key_SECURE_xyz
WEBHOOK_URL=https://api.example.com/webhook

# Feature Flags
ENABLE_ANALYTICS=true
ENABLE_LOGGING=true

# Security Settings (NEW from upstream)
RATE_LIMIT_ENABLED=true
RATE_LIMIT_REQUESTS=100

# Monitoring (NEW from upstream)
SENTRY_DSN=
METRICS_ENABLED=false

# Storage (NEW from upstream)
STORAGE_BACKEND=local
STORAGE_PATH=/data/uploads
```

## Key Takeaways

- **Environment isolation**: Each environment file maintains its own customizations
- **Security preserved**: API keys and sensitive data stay unchanged
- **New settings inherited**: Upstream's new security, monitoring, and storage settings automatically added
- **Selective application**: Use `patch-md diff <file>` to check specific environments

## Advanced: Per-Environment Reconciliation

```bash
# Check development environment only
patch-md diff config/development.env

# Check production environment only
patch-md diff config/production.env

# Reconcile production with upstream
patch-md apply --force config/production.env
```

## Commands Used

```bash
patch-md init --target "upstream@v1.5.0" --author "devops-team"
patch-md add config/development.env --original .original/.env.example
patch-md add config/production.env --original .original/.env.example
patch-md diff config/production.env
patch-md status
patch-md apply --force
```