//! Application-level module exports.
//!
//! This folder is the equivalent of the NestJS "app" folder,
//! containing the root AppModule along with optional
//! application-wide controllers and services.

pub mod app_module;
pub mod app_controller;
pub mod app_service;

pub use app_module::*;
pub use app_controller::*;
pub use app_service::*;