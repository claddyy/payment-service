use actix_web::{get, HttpResponse, Responder};

use crate::middlewares::auth::JWTClaim;

#[get("")]
pub async fn healthcheck(_claim: JWTClaim) -> impl Responder {
    HttpResponse::Ok().body("OK")
}
