# cargo-smith Tutorial: Build an Actix Web API with JWT Auth

This tutorial walks you through creating a Rust web API from scratch using `cargo-smith`, enabling JWT authentication, adding a feature, and protecting routes with role guards.

**Prerequisites:** Rust 1.85+ installed. MongoDB optional (for the database examples).

---

## Step 1: Install cargo-smith

```bash
cargo install cargo-smith
```

Verify installation:

```bash
cargo-smith --version
```

---

## Step 2: Create a new project

### Interactive mode

```bash
cargo-smith new
```

You'll be prompted for:
- **Project name** (e.g., `my-api`)
- **Template type** (choose "Modular (Actix-Web)")
- **Enable JWT authentication?** (yes/no)
- **Use local cargo-smith source?** (yes only if developing cargo-smith itself; otherwise no)

### Non-interactive mode

```bash
# Without auth
cargo-smith new my-api modular

# With JWT auth scaffolding
cargo-smith new my-api modular --auth

# With auth + local cargo-smith (for development)
cargo-smith new my-api modular --auth --local
```

The `--auth` flag generates an `auth` feature with a `/auth/login` endpoint and wires `JwtMiddleware` into the server.

Navigate into the project:

```bash
cd my-api
```

---

## Step 3: Explore the generated structure

```
my-api/
├── .cargo-smith            # Project metadata (tracks generated features)
├── .env.example            # Environment variable template
├── Cargo.toml
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library root
│   ├── server/
│   │   ├── mod.rs
│   │   └── server.rs       # HTTP server + middleware
│   ├── features/
│   │   ├── mod.rs          # Feature registry ([SMITH-MOD], [SMITH-INIT])
│   │   └── auth/           # (only if --auth)
│   │       ├── mod.rs
│   │       ├── controller.rs
│   │       ├── service.rs
│   │       ├── model.rs
│   │       └── routes.rs
│   └── utils/
│       └── mod.rs
```

---

## Step 4: Configure environment

Copy the example env file and set your secrets:

```bash
cp .env.example .env
```

Edit `.env`:

```env
JWT_SECRET=your-super-secure-jwt-secret-key-change-this-in-production
ENCRYPTION_KEY=your-encryption-passphrase
```

> **Important:** Replace the placeholder secrets before deploying. Generate a secure key with `openssl rand -base64 64`.

---

## Step 5: Run the server

```bash
cargo run
```

You should see:

```
Starting Actix Web server on http://127.0.0.1:8080
```

Test the health check:

```bash
curl http://127.0.0.1:8080/health
# → OK
```

---

## Step 6: Add a feature

Generate a `users` feature module:

```bash
cargo-smith add feature users
```

This creates `src/features/users/` with:
- `mod.rs` — feature entry and init
- `controller.rs` — request handlers
- `service.rs` — business logic
- `model.rs` — data structures
- `routes.rs` — route definitions

And automatically registers it in `src/features/mod.rs`:

```rust
// [SMITH-MOD]
pub mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    // [SMITH-INIT]
    users::init(cfg);
}
```

---

## Step 7: Implement the feature

### Model (`src/features/users/model.rs`)

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}
```

### Service (`src/features/users/service.rs`)

```rust
use crate::features::users::model::User;

pub async fn get_all() -> Vec<User> {
    // Replace with real database query
    vec![]
}

pub async fn get_by_id(id: &str) -> Option<User> {
    // Replace with real database lookup
    None
}

pub async fn create(data: User) -> User {
    // Replace with real database insert
    data
}
```

### Controller (`src/features/users/controller.rs`)

```rust
use actix_web::{web, Responder};
use cargo_smith::common::response::ApiResponse;
use cargo_smith::common::extractors::validated::ValidatedJson;
use crate::features::users::service;
use crate::features::users::model::User;

pub async fn get_all() -> impl Responder {
    let data = service::get_all().await;
    ApiResponse::ok("users retrieved", data)
}

pub async fn get_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match service::get_by_id(&id).await {
        Some(item) => ApiResponse::ok(format!("User {} retrieved", id), item),
        None => ApiResponse::not_found(format!("User {} not found", id)),
    }
}

pub async fn create(body: ValidatedJson<User>) -> impl Responder {
    let new_item = service::create(body.into_inner()).await;
    ApiResponse::created("user created", new_item)
}
```

### Routes (`src/features/users/routes.rs`)

```rust
use actix_web::web;
use crate::features::users::controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(controller::get_all))
            .route("", web::post().to(controller::create))
            .route("/{id}", web::get().to(controller::get_by_id))
    );
}
```

Rebuild and test:

```bash
cargo run
curl http://127.0.0.1:8080/users
# → {"status_code":200,"message":"users retrieved","data":[]}
```

---

## Step 8: Use JWT authentication

If you created the project with `--auth`, you already have an auth feature. Let's test it.

### Login to get a token

```bash
curl -X POST http://127.0.0.1:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "secret"}'
```

Response:

```json
{
  "status_code": 200,
  "message": "login successful",
  "data": {
    "access_token": "eyJ...",
    "refresh_token": "eyJ..."
  }
}
```

### Protect a route with a guard

Add a guard to your controller:

```rust
use cargo_smith::common::guards::{RequireAuth, RequireAdmin};
use cargo_smith::common::guards::GuardClaims;
use cargo_smith::common::response::ApiResponse;

// Any logged-in user
#[get("/me")]
async fn me(guard: RequireAuth) -> impl Responder {
    ApiResponse::ok("me", guard.username())
}

// Admin only
#[get("/admin")]
async fn admin(guard: RequireAdmin) -> impl Responder {
    ApiResponse::ok("admin area", guard.username())
}
```

Register the routes in `routes.rs` as usual.

### Call a protected route

```bash
TOKEN="eyJ..."  # from login response

# Works (any valid token)
curl http://127.0.0.1:8080/me -H "Authorization: Bearer $TOKEN"

# Fails with 403 (token has "user" role, not "admin")
curl http://127.0.0.1:8080/admin -H "Authorization: Bearer $TOKEN"
```

### Multiple roles

```rust
use cargo_smith::common::guards::AnyOf;
use cargo_smith::common::guards::AuthGuard;
use cargo_smith::common::roles::{Admin, User};

type RequireStaff = AuthGuard<AnyOf<(Admin, User)>>;

#[get("/staff")]
async fn staff(guard: RequireStaff) -> impl Responder {
    ApiResponse::ok("staff area", guard.username())
}
```

### Custom roles

```rust
use cargo_smith::define_custom_roles;

define_custom_roles! {
    AppRoles {
        Editor => EditorRole,
        Viewer => ViewerRole,
    }
}

// Use AuthGuard::<EditorRole> as a guard type
```

---

## Step 9: Customize the auth service

The generated `auth/service.rs` hard-codes the role to `"user"`:

```rust
let role = "user".to_string(); // TODO: look up from database
```

In a real app, verify credentials against your database and fetch the user's actual role:

```rust
pub async fn login(req: LoginRequest) -> LoginResponse {
    // 1. Look up user by username in DB
    // 2. Verify password with bcrypt
    // 3. Get the user's role from DB
    let role = "admin".to_string(); // from DB lookup

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me".to_string());

    // issue token with the real role...
}
```

---

## Step 10: Common patterns

### Paginated list endpoint

```rust
use cargo_smith::common::extractors::pagination::{PaginationQuery, PaginatedResponse};

#[get("/users")]
async fn list_users(query: web::Query<PaginationQuery>) -> impl Responder {
    let q = query.into_inner();
    let items = vec![]; // fetch from DB
    let total = 0u64;  // count from DB
    ApiResponse::ok("users", PaginatedResponse::new(items, total, &q))
}
```

### Validated request body

```rust
use cargo_smith::common::extractors::validated::ValidatedJson;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[post("/users")]
async fn create(body: ValidatedJson<CreateUser>) -> impl Responder {
    let data = body.into_inner();
    ApiResponse::created("user created", data)
}
```

### Error handling

```rust
use cargo_smith::common::errors::AppError;

async fn get_user(id: &str) -> Result<User, AppError> {
    // AppError maps to proper HTTP status codes automatically
    Err(AppError::NotFound(format!("user {} not found", id)))
}
```

---

## Step 11: Development workflow

```bash
# Check formatting
cargo fmt --check

# Lint
cargo clippy

# Run tests
cargo test

# Build release
cargo build --release
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `cargo-smith: command not found` | Run `cargo install cargo-smith` or add `~/.cargo/bin` to PATH |
| Generated project won't compile | Ensure `cargo-smith` is installed (`cargo install cargo-smith`) or use `--local` |
| `Not a cargo-smith project` | Run `cargo-smith new` first to create `.cargo-smith` metadata file |
| Feature already exists | Delete `src/features/<name>/` manually and remove from `.cargo-smith` |
| JWT validation fails | Check `JWT_SECRET` in `.env` matches between server and token issuance |

---

## Next steps

- Replace placeholder logic in `service.rs` with real database queries
- Configure CORS for your frontend origin (currently allows any origin)
- Add TLS for production (use a reverse proxy like nginx or configure Actix TLS)
- Set up rate limiting middleware
- Add more features with `cargo-smith add feature <name>`

For the full API reference of the `cargo-smith` library, see [DOCUMENTATION.MD](DOCUMENTATION.MD).