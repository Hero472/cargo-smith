use std::env;
use dotenvy::dotenv;

/// Application configuration helper backed by environment variables.
///
/// Loads a `.env` file on init and provides typed accessors for env vars.
#[derive(Debug, Clone, Copy, Default)]
pub struct Config;

impl Config {
    /// Loads the `.env` file into the process environment.
    ///
    /// Call this at the very top of `main`. Silently ignores missing `.env`
    /// files (common in production where real env vars are set).
    pub fn init() {
        if let Err(e) = dotenv() {
            // We don't panic here because in Production (Docker), 
            // .env files often don't exist—real env vars are used instead.
            eprintln!("Note: No .env file loaded: {}", e);
        }
    }

    /// Returns the value of the environment variable `key`.
    ///
    /// # Panics
    /// Panics if the variable is not set.
    pub fn get(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable [{}] is missing!", key))
    }

    /// Returns the value of `key`, or `default` if it is not set.
    pub fn get_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Returns `key` parsed into type `T`.
    ///
    /// # Panics
    /// Panics if the variable is missing or cannot be parsed as `T`.
    ///
    /// # Examples
    /// ```ignore
    /// let port: u16 = Config::get_as("PORT");
    /// ```
    pub fn get_as<T>(key: &str) -> T 
    where 
        T: std::str::FromStr, 
        <T as std::str::FromStr>::Err: std::fmt::Debug 
    {
        Self::get(key)
            .parse::<T>()
            .unwrap_or_else(|_| panic!("Environment variable [{}] is not the correct type!", key))
    }

    /// Returns `true` if the environment variable `key` is set.
    pub fn has(key: &str) -> bool {
        env::var(key).is_ok()
    }
}