use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use crate::features::auth::model::{LoginRequest, LoginResponse};

#[derive(Serialize)]
struct Claims {
    sub: String,
    username: String,
    /// Role string used by cargo-smith's role guards (e.g. "admin", "user").
    role: String,
    exp: usize,
}

pub async fn login(req: LoginRequest) -> LoginResponse {
    // NOTE: verify credentials against your database here before issuing tokens.
    // The role is hard-coded to "user" for scaffolding; look it up from your
    // database in a real implementation.
    let role = "user".to_string();

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me-in-production".to_string());

    let now = Utc::now();
    let access_claims = Claims {
        sub: "user-id".to_string(),
        username: req.username.clone(),
        role: role.clone(),
        exp: (now + Duration::minutes(15)).timestamp() as usize,
    };
    let refresh_claims = Claims {
        sub: "user-id".to_string(),
        username: req.username,
        role,
        exp: (now + Duration::days(7)).timestamp() as usize,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default();
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default();

    LoginResponse {
        access_token,
        refresh_token,
    }
}