use std::error::Error;
use serde::{Serialize, Deserialize};
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};
use pbkdf2::pbkdf2_hmac;
use base64::Engine;
use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;
use bcrypt::{hash as crypt_hash, DEFAULT_COST};

use crate::{DefaultRoles, auth::claims::Claims};

/// Derives a 32‑byte key from a password‑like string using PBKDF2-HMAC-SHA256.
///
/// This is a convenience helper for turning a user‑supplied encryption secret
/// into a fixed‑size key suitable for AES‑256-GCM. A high iteration count and
/// a constant salt provide resistance against brute-force and rainbow-table
/// attacks. Prefer a per-user random salt in production code.
/// Fixed application salt used for key derivation. A per-user salt would be
/// preferable, but `AuthService` derives a key from a passphrase without
/// storing extra state, so a constant salt is used. Combined with a high
/// iteration count this is a meaningful improvement over a single SHA-256.
const KEY_DERIVATION_SALT: &[u8] = b"cargo-smith-aes-key-derivation-salt-v1";
const KEY_DERIVATION_ITERATIONS: u32 = 100_000;

fn derive_key_from_string(key_str: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(key_str.as_bytes(), KEY_DERIVATION_SALT, KEY_DERIVATION_ITERATIONS, &mut key);
    key
}

/// Service handling password hashing (bcrypt), JWT creation/verification,
/// and AES-256-GCM encryption/decryption.
///
/// # Fields
/// * `secret_key` – secret used for signing JWTs (HMAC‑SHA256).
/// * `encryption_key` – secret from which a 256‑bit AES key is derived.
///
/// # Security
///
/// Both secrets are stored in plain text when serialized — use with care.
/// Do not persist this struct to disk or logs.
#[derive(Serialize, Deserialize)]
pub struct AuthService {
    secret_key: String,
    encryption_key: String,
}

impl AuthService {

    /// Creates a new `AuthService` with the given secrets.
    ///
    /// # Parameters
    /// * `secret_key`  – the HMAC secret for signing JWTs.
    /// * `encryption_key` – a passphrase or key material that will be hashed
    ///   with SHA‑256 to produce the AES‑256 encryption key.
    pub fn new(secret_key: String, encryption_key: String) -> Self {
        Self {
            secret_key,
            encryption_key,
        }
    }

    /// Hashes an arbitrary string with SHA-256 and returns the hex‑encoded
    /// digest.
    ///
    /// The input is fed to the hasher twice (once as a prefix and once as an
    /// update), which is **not** a standard construction and provides no
    /// additional security over a single hash. This method exists for
    /// non-security checksum use cases only.
    ///
    /// This is **not** suitable for password storage – see
    /// [`hash_password`](Self::hash_password) for that purpose.
    ///
    /// # Examples
    /// ```
    /// let digest = cargo_smith::AuthService::hash("hello").unwrap();
    /// assert_eq!(digest.len(), 64);
    /// ```
    pub fn hash(input: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha256::new_with_prefix(input.as_bytes());
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Hashes a password using bcrypt with the default cost factor.
    ///
    /// This is the recommended method for storing user passwords.
    ///
    /// # Errors
    /// Returns a `BcryptError` if the hashing process fails (e.g., extremely
    /// long input or system randomness issues).
    pub fn hash_password(input: &str) -> Result<String, bcrypt::BcryptError> {
        crypt_hash(input, DEFAULT_COST)
    }

    /// Verifies a password against a bcrypt hash.
    ///
    /// # Errors
    /// Returns a `BcryptError` if the hash is malformed or other internal
    /// errors occur.
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, hash)
    }

    /// Generates a signed JWT containing arbitrary payload data.
    ///
    /// # Errors
    /// Returns an error if encoding fails (e.g., serialisation problem).
    ///
    /// # Examples
    /// ```
    /// use cargo_smith::AuthService;
    /// use chrono::Duration;
    /// let auth = AuthService::new("secret".to_string(), "enc".to_string());
    /// let token = auth.generate_token(
    ///     "user@example.com".to_string(),
    ///     serde_json::json!({"role": "admin"}),
    ///     Duration::minutes(15),
    /// ).unwrap();
    /// assert!(!token.is_empty());
    /// ```
    pub fn generate_token<T: Serialize>(
        &self,
        sub: String,
        data: T,
        ttl: chrono::Duration,
    ) -> Result<String, Box<dyn Error>> {
        let claims = Claims::with_expiration(sub, data, ttl);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
        .map_err(|e| format!("JWT encoding failed: {e}").into())
    }

    /// Checks whether a JWT is valid (signature, expiration, etc.).
    ///
    /// Returns `true` if the token can be decoded and passes all validation
    /// checks.
    ///
    /// The generic `T` must match the data type that was originally encoded.
    pub fn verify_token<T>(&self, token: &str) -> bool
    where
        T: for<'de> Deserialize<'de> + Clone, 
    {
        let validation = Validation::default();
        let result = decode::<Claims<T>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &validation,
        );
        result.is_ok()
    }

    /// Returns `true` if the token has expired (or is invalid).
    ///
    /// Invalid tokens (bad signature, malformed, etc.) are treated as expired.
    pub fn is_token_expired<T>(&self, token: &str) -> bool 
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let validation = Validation::default();
        if let Ok(data) = decode::<Claims<T>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &validation,
        ) {
            let now = usize::try_from(Utc::now().timestamp()).unwrap_or(0);
            data.claims.exp < now
        } else {
            true // Treat invalid token as expired
        }
    }

    /// Encrypts a plaintext string using AES‑256‑GCM.
    ///
    /// A random 96‑bit nonce is generated for every encryption. The result is
    /// a base64‑encoded blob containing the nonce followed by the ciphertext.
    ///
    /// # Errors
    /// Returns an error if encryption or base64 encoding fails.
    pub fn encrypt(&self, input: &str) -> Result<String, Box<dyn Error>> {

        let key_bytes = derive_key_from_string(&self.encryption_key);
        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let cipher_text = cipher.encrypt(&nonce, input.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&cipher_text);

        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_data))
    }

    /// Decrypts a base64‑encoded AES‑256‑GCM ciphertext produced by
    /// [`encrypt`](Self::encrypt).
    ///
    /// # Errors
    /// Fails if the input is not valid base64, is too short to contain a
    /// nonce, or if the authentication tag does not match (wrong key or
    /// corrupted data).
    pub fn decrypt(&self, input: &str) -> Result<String, Box<dyn Error>> {

        let key_bytes = derive_key_from_string(&self.encryption_key);
        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let encrypted_data = base64::engine::general_purpose::STANDARD.decode(input)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;

        if encrypted_data.len() < 12 {
            return Err("Invalid encrypted data: too short".into());
        }
        
        let (nonce_bytes, cipher_text) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher.decrypt(nonce, cipher_text)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8: {}", e).into())
    }

    /// Creates an access/refresh token pair for a registered user.
    ///
    /// * Access token – valid for 15 minutes, payload contains `username` and
    ///   `kind: "registered"`.
    /// * Refresh token – valid for 7 days, payload contains `username` and
    ///   `type: "refresh"`.
    pub fn token_pair(&self, id: &str, username: &str, role: DefaultRoles) -> (String, String) {
        let access = self.generate_token(
            id.to_string(),
            serde_json::json!({ "username": username, "role": role }),
            chrono::Duration::minutes(15),
        ).unwrap();
        let refresh = self.generate_token(
            id.to_string(),
            serde_json::json!({ "username": username, "role": role, "type": "refresh" }),
            chrono::Duration::days(7),
        ).unwrap();
        (access, refresh)
    }

    /// Creates an access/refresh token pair for a guest user.
    ///
    /// * Access token – valid for 15 **hours** (instead of minutes), payload
    ///   contains `username` and `kind: "guest"`.
    /// * Refresh token – valid for 1 hour, payload includes `kind: "guest"`
    ///   and `type: "refresh"`.
    ///
    /// Guest tokens are intentionally short‑lived because guest accounts are
    /// meant to be ephemeral.
    pub fn guest_token_pair(
        &self,
        id: &str,
        username: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        let access = self.generate_token(
            id.to_string(),
            serde_json::json!({ "username": username, "kind": "guest" }),
            chrono::Duration::hours(15),
        )?;
        let refresh = self.generate_token(
            id.to_string(),
            serde_json::json!({ "username": username, "kind": "guest", "type": "refresh" }),
            chrono::Duration::hours(1),
        )?;
        Ok((access, refresh))
    }
}