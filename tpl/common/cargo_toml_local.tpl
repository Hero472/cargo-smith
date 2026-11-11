[package]
name = "{{name}}"
version = "0.1.0"
edition = "2024"

[dependencies]
cargo-smith = { path = "{{cargo_smith_path}}" }
actix-web = "4.13"
actix-cors = "0.7"
tokio = { version = "1.50", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.149"
validator = { version = "0.20", features = ["derive"] }
jsonwebtoken = "9"
chrono = "0.4"

[lib]
name = "{{name_snake_case}}"
path = "src/lib.rs"