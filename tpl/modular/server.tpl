// Server configuration and startup
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use cargo_smith::ApiInterceptor;

use crate::features::init;

/// Starts the HTTP server and begins listening for requests
pub async fn run() -> std::io::Result<()> {
    println!("Starting Actix Web server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(ApiInterceptor)
            .route("/health", web::get().to(|| async { "OK" }))
            .configure(init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}