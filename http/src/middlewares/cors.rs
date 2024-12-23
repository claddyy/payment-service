#![allow(unused)]

use std::env;

use actix_cors::Cors;
use actix_web::http;

/// CORS configuration middleware
pub fn cors() -> Cors {
    Cors::default()
        // TODO: Refactor this for Frontend Origin
        // .allowed_origin(&frontend_origin)
        .send_wildcard()
        .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
