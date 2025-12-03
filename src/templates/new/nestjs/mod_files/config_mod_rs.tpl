//! Application configuration utilities.
//!
//! This is the equivalent of NestJS "config/".
//! Typical contents:
//! - environment variable loader
//! - global settings structs
//! - configuration helpers

pub mod env;
pub mod settings;

pub use env::*;
pub use settings::*;