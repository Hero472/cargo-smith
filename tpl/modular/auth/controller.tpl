use cargo_smith::common::response::ApiResponse;
use cargo_smith::common::extractors::validated::ValidatedJson;
use crate::features::auth::service;
use crate::features::auth::model::LoginRequest;

pub async fn login(body: ValidatedJson<LoginRequest>) -> impl actix_web::Responder {
    let response = service::login(body.into_inner()).await;
    ApiResponse::ok("login successful", response)
}