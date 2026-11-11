# cargo-smith

[![Crates.io](https://img.shields.io/crates/v/cargo-smith.svg)](https://crates.io/crates/cargo-smith)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

A high-performance CLI tool for scaffolding modular Rust & Actix Web applications.

Inspired by the scalability of [NestJS](https://nestjs.com/), built with the power of Rust.

## Why cargo-smith?

- **Modular by Design** — organizes code by domain features, so projects stay maintainable from microservice to enterprise.
- **Zero-Boilerplate** — generate a feature (controller, service, model, routes) in one command.
- **Developer Experience** — automatic Actix-Web `ServiceConfig` injection; no manual route imports.
- **Production-Ready Foundations** — JWT auth, role-based guards, MongoDB helpers, pagination, validation, and structured responses out of the box.
- **Optimized** — Rust-level performance with clean binary footprints.

## Installation

### From crates.io

```bash
cargo install cargo-smith
```

### From source

```bash
git clone https://github.com/Hero472/cargo-smith.git
cd cargo-smith
cargo install --path .
```

**Requirements:** Rust 1.85+ (2024 edition).

## Usage

### Create a new project

```bash
# Interactive mode (prompts for name and template)
cargo-smith new

# Non-interactive
cargo-smith new my-project --template-type modular
```

Generated projects use the **Modular (Actix-Web)** template by default and include a `.cargo-smith` metadata file tracking your project state.

### Add a feature

```bash
# Adds a 'users' module with controller, service, model, and routes
cargo-smith add feature users

# Shortcut aliases
cargo-smith g feature users
cargo-smith generate feature users
```

### Help

```bash
cargo-smith --help
cargo-smith --version
```

## Generated project structure

```
my-project/
├── .cargo-smith            # Project metadata and feature registry
├── .env.example            # Environment configuration template
├── Cargo.toml
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library root
│   ├── server/
│   │   └── server.rs       # HTTP server startup (127.0.0.1:8080)
│   ├── utils/              # Shared utilities
│   └── features/           # Domain logic (the core)
│       ├── mod.rs          # Central registry [SMITH-MOD]
│       └── users/          # Generated feature
│           ├── mod.rs      # Feature entry & init
│           ├── controller.rs
│           ├── service.rs
│           ├── model.rs
│           └── routes.rs
```

Each generated feature is automatically registered in `features/mod.rs` via marker-based injection (`[SMITH-MOD]`, `[SMITH-INIT]`), so routes are wired up without manual edits.

## What's included in generated apps

- **Actix Web 4** with Tokio runtime
- **Feature-based modules** for routes, handlers, and server configuration
- **JWT Authentication** with middleware protection (`JwtMiddleware`)
- **Role-based guards** (`RequireAdmin`, `RequireUser`, `RequireGuest`, `AnyOf`)
- **MongoDB helpers** (`MongoCollection`, `Db`, pagination, transactions)
- **Validated JSON** extractor (`ValidatedJson<T>` with `validator`)
- **Pagination** (`PaginationQuery`, `PaginatedResponse`)
- **Standardized responses** (`ApiResponse<T>`)
- **Error handling** (`AppError` with HTTP status mapping)
- **Environment config** via `dotenvy`

## Architecture

cargo-smith itself is split into a library (`src/lib.rs`) and a thin binary (`src/main.rs`):

```
src/
├── main.rs          # CLI parsing + interactive prompts
├── lib.rs           # Public API re-exports
├── commands/        # `new`, `feature` command handlers
├── templates/       # Template engine + Modular template
│   └── modular/     # generate.rs, feature.rs (with path validation)
├── auth/            # AuthService (JWT, bcrypt, AES-GCM)
├── common/          # errors, guards, roles, middleware, extractors, response
├── config/          # Environment configuration
├── database/        # MongoDB collection helpers
└── utils/           # String helpers (to_pascal_case, etc.)
```

Feature names are validated (`^[a-z][a-z0-9_-]*$`) to prevent path traversal and keep generated module names valid.

## Development

### Build

```bash
cargo build
```

### Run locally

```bash
cargo run -- new my-test-project
```

### Test

```bash
cargo test          # unit + integration tests
cargo test --doc    # doctests
cargo clippy        # lint (correctness = deny)
cargo fmt --check   # formatting
```

### MSRV

Rust 1.85+ (2024 edition).

## Security notes

- Generated projects bind to `127.0.0.1:8080` by default — configure `0.0.0.0` and TLS for production.
- Replace the placeholder `JWT_SECRET` in `.env` before deploying.
- AES keys are derived via PBKDF2-HMAC-SHA256 (100k iterations). For production, prefer a per-user random salt.
- Feature names are validated to prevent path traversal.

## Roadmap

- [x] Modular template (feature-based organization)
- [x] Automatic feature registration (`features/mod.rs`)
- [ ] CRUD generation (instant database integration)
- [ ] Routing framework (macro-based routing)
- [ ] Additional templates (web-api, cli-tool, library, microservice)

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Run `cargo fmt`, `cargo clippy`, and `cargo test`
4. Open a pull request

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

MIT — see [LICENSE](LICENSE) for details.

## Author

Pedro Herrera ([@Hero472](https://github.com/Hero472))