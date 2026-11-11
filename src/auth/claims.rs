use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

/// JWT claims payload with subject, issued-at, expiration, and custom data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claims<T = serde_json::Value> {
    /// Subject (whom the token refers to)
    pub sub: String,
    /// Issued at (timestamp)
    pub iat: usize,
    /// Expiration time (timestamp) 
    pub exp: usize,
    /// Custom claims data
    pub data: T,
}

impl<T> Claims<T> {
    /// Creates a new `Claims` instance with an expiration relative to **now**.
    ///
    /// * `sub`  – subject identifier (e.g., user email).
    /// * `data` – custom payload.
    /// * `ttl`  – lifetime of the token from the moment of creation.
    ///
    /// The `iat` and `exp` fields are set automatically based on the current UTC time.
    pub fn with_expiration(sub: String, data: T, ttl: Duration) -> Self {
        let now = Utc::now();
        let iat = usize::try_from(now.timestamp()).unwrap_or(0);
        let exp = usize::try_from((now + ttl).timestamp()).unwrap_or(0);
        Self { sub, iat, exp, data }
    }
}