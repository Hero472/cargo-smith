// Request handlers for the Actix Web application
use actix_web::{HttpResponse, Responder};

/// Simple hello world endpoint
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World! from Actix Web")
}