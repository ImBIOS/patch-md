# Scenario 4: Multi-File Coordinated Changes

## Problem Statement

You need to modify multiple files that work together - for example, updating an API client library and all the places that call it. When upstream updates the library, you need all your changes to be applied consistently.

## Project Structure

```
project/
├── src/
│   ├── api/
│   │   ├── client.rs       # API client library
│   │   └── models.rs       # Data models
│   ├── handlers/
│   │   ├── users.rs        # User endpoints
│   │   └── orders.rs       # Order endpoints
│   └── main.rs
├── PATCH.md
└── .original/
```

## Setup

```bash
mkdir -p scenario-4/src/api scenario-4/src/handlers scenario-4/.original

# Original API client
cat > src/api/client.rs << 'EOF'
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, reqwest::Error> {
        self.client
            .get(&format!("{}/users", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_orders(&self, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
        self.client
            .get(&format!("{}/users/{}/orders", self.base_url, user_id))
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total: f64,
    pub status: String,
}
EOF

# Original handlers
cat > src/handlers/users.rs << 'EOF'
use crate::api::client::{ApiClient, User};

pub async fn list_users(client: &ApiClient) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let users = client.get_users().await?;
    Ok(users)
}

pub async fn get_user_by_id(client: &ApiClient, id: i32) -> Result<Option<User>, reqwest::Error> {
    let users = client.get_users().await?;
    Ok(users.into_iter().find(|u| u.id == id))
}
EOF

cat > src/handlers/orders.rs << 'EOF'
use crate::api::client::{ApiClient, Order};

pub async fn list_user_orders(client: &ApiClient, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
    client.get_orders(user_id).await
}

pub async fn get_order_total(client: &ApiClient, user_id: i32) -> Result<f64, reqwest::Error> {
    let orders = client.get_orders(user_id).await?;
    Ok(orders.iter().map(|o| o.total).sum())
}
EOF

# Save originals
cp -r src .original/
```

## Your Customizations

You need to:
1. Add retry logic to the API client
2. Add caching to the client
3. Update handlers to use the new caching features

### Enhanced API Client (src/api/client.rs)

```bash
cat > src/api/client.rs << 'EOF'
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ApiClient {
    base_url: String,
    client: Client,
    cache: RwLock<HashMap<String, (String, std::time::Instant)>>,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            cache: RwLock::new(HashMap::new()),
            timeout: Duration::from_secs(30),
        }
    }

    async fn get_with_retry(&self, url: &str, retries: u32) -> Result<Response, reqwest::Error> {
        let mut last_error = None;

        for attempt in 0..=retries {
            match self.client.get(url).send().await {
                Ok(response) if response.status().is_success() => return Ok(response),
                Ok(response) if response.status().is_server_error() && attempt < retries => {
                    last_error = Some(response.text().await.unwrap_or_default());
                    tokio::time::sleep(Duration::from_millis(100 * 2_u64.pow(attempt))).await;
                }
                Ok(response) => return Err(response.error_for_status().unwrap_err()),
                Err(e) if attempt < retries => {
                    last_error = Some(e.to_string());
                    tokio::time::sleep(Duration::from_millis(100 * 2_u64.pow(attempt))).await;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or("Max retries exceeded".into()).into())
    }

    pub async fn get_users(&self) -> Result<Vec<User>, reqwest::Error> {
        let url = format!("{}/users", self.base_url);

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some((data, timestamp)) = cache.get(&url) {
                if timestamp.elapsed() < Duration::from_secs(60) {
                    return serde_json::from_str(data).map_err(|e| e.into());
                }
            }
        }

        let response = self.get_with_retry(&url, 3).await?;
        let data = response.text().await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(url, (data.clone(), std::time::Instant::now()));
        }

        serde_json::from_str(&data)
    }

    pub async fn get_orders(&self, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
        let url = format!("{}/users/{}/orders", self.base_url, user_id);
        let response = self.get_with_retry(&url, 3).await?;
        response.json().await
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total: f64,
    pub status: String,
}
EOF
```

### Updated Handlers (src/handlers/users.rs)

```bash
cat > src/handlers/users.rs << 'EOF'
use crate::api::client::{ApiClient, User};

pub async fn list_users(client: &ApiClient) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    // Uses the new retry and cache features automatically
    let users = client.get_users().await?;
    Ok(users)
}

pub async fn get_user_by_id(client: &ApiClient, id: i32) -> Result<Option<User>, reqwest::Error> {
    let users = client.get_users().await?;
    Ok(users.into_iter().find(|u| u.id == id))
}

pub async fn search_users_by_name(client: &ApiClient, query: &str) -> Result<Vec<User>, reqwest::Error> {
    let users = client.get_users().await?;
    Ok(users.into_iter()
        .filter(|u| u.name.to_lowercase().contains(&query.to_lowercase()))
        .collect())
}
EOF

cat > src/handlers/orders.rs << 'EOF'
use crate::api::client::{ApiClient, Order};

pub async fn list_user_orders(client: &ApiClient, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
    // Uses the new retry features automatically
    client.get_orders(user_id).await
}

pub async fn get_order_total(client: &ApiClient, user_id: i32) -> Result<f64, reqwest::Error> {
    let orders = client.get_orders(user_id).await?;
    Ok(orders.iter().map(|o| o.total).sum())
}

pub async fn get_pending_orders(client: &ApiClient, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
    let orders = client.get_orders(user_id).await?;
    Ok(orders.into_iter().filter(|o| o.status == "pending").collect())
}
EOF
```

## Tracking All Changes with patch-md

```bash
# Initialize
patch-md init --target "my-api-client@v2.0.0" --author "platform-team"

# Add all modified files
patch-md add src/api/client.rs --original .original/src/api/client.rs
patch-md add src/handlers/users.rs --original .original/src/handlers/users.rs
patch-md add src/handlers/orders.rs --original .original/src/handlers/orders.rs
```

### Generated PATCH.md

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | my-api-client@v2.0.0 |
| created | 2026-04-24T12:00:00Z |
| author | platform-team |

## Patches

### src/api/client.rs

```diff
--- a/src/api/client.rs
+++ b/src/api/client.rs
@@ -1,5 +1,8 @@
 use reqwest::Client;
 use serde::{Deserialize, Serialize};
+use std::time::Duration;
+use std::collections::HashMap;
+use tokio::sync::RwLock;

 pub struct ApiClient {
-    base_url: String,
+    base_url: String,
     client: Client,
+    cache: RwLock<HashMap<String, (String, std::time::Instant)>>,
+    timeout: Duration,
 }
```

### src/handlers/users.rs

```diff
--- a/src/handlers/users.rs
+++ b/src/handlers/users.rs
@@ -8,3 +8,10 @@ pub async fn get_user_by_id(client: &ApiClient, id: i32) -> Result<Option<User>
     let users = client.get_users().await?;
     Ok(users.into_iter().find(|u| u.id == id))
 }
+
+pub async fn search_users_by_name(client: &ApiClient, query: &str) -> Result<Vec<User>, reqwest::Error> {
+    let users = client.get_users().await?;
+    Ok(users.into_iter()
+        .filter(|u| u.name.to_lowercase().contains(&query.to_lowercase()))
+        .collect())
+}
```

### src/handlers/orders.rs

```diff
--- a/src/handlers/orders.rs
+++ b/src/handlers/orders.rs
@@ -12,3 +12,8 @@ pub async fn get_order_total(client: &ApiClient, user_id: i32) -> Result<f64, re
     let orders = client.get_orders(user_id).await?;
     Ok(orders.iter().map(|o| o.total).sum())
 }
+
+pub async fn get_pending_orders(client: &ApiClient, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
+    let orders = client.get_orders(user_id).await?;
+    Ok(orders.into_iter().filter(|o| o.status == "pending").collect())
+}
```

```

## Upstream Updates the Library

```bash
# Upstream adds new endpoints and fixes a bug
cat > .original/src/api/client.rs << 'EOF'
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    // NEW: Products endpoint
    pub async fn get_products(&self) -> Result<Vec<Product>, reqwest::Error> {
        self.client
            .get(&format!("{}/products", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    // BUG FIX: Added proper error handling
    pub async fn get_users(&self) -> Result<Vec<User>, reqwest::Error> {
        let response = self.client
            .get(&format!("{}/users", self.base_url))
            .send()
            .await?;

        if response.status() == 401 {
            return Err(reqwest::Error::from(
                std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Unauthorized")
            ));
        }

        response.json().await
    }

    pub async fn get_orders(&self, user_id: i32) -> Result<Vec<Order>, reqwest::Error> {
        self.client
            .get(&format!("{}/users/{}/orders", self.base_url, user_id))
            .send()
            .await?
            .json()
            .await
    }
}
EOF
```

## Reconciliation

```bash
# Check status - see all coordinated changes
patch-md status

# Output:
# PATCH.md Status
# ================
# Target: my-api-client@v2.0.0
# Created: 2026-04-24T12:00:00Z
# Patches: 3
#
# [MODIFIED] src/api/client.rs
# [MODIFIED] src/handlers/users.rs
# [MODIFIED] src/handlers/orders.rs

# Apply all coordinated changes
patch-md apply --force
```

### Final Result

Your enhanced API client with:
- Retry logic
- Caching
- Custom timeout

Combined with upstream's:
- New products endpoint
- Bug fixes

And all your handler updates preserved.

## Key Takeaways

- **Coordinated changes**: All related files tracked together
- **Consistent application**: Either all changes apply or none
- **Selective viewing**: Check specific files with `patch-md diff <file>`
- **New upstream features**: Automatically inherited (like the new `get_products` endpoint)

## Commands Used

```bash
patch-md init --target "my-api-client@v2.0.0" --author "platform-team"
patch-md add src/api/client.rs --original .original/src/api/client.rs
patch-md add src/handlers/users.rs --original .original/src/handlers/users.rs
patch-md add src/handlers/orders.rs --original .original/src/handlers/orders.rs
patch-md status
patch-md diff src/api/client.rs
patch-md apply --force
```