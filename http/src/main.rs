mod app_state;
mod constants;
mod features;
mod middlewares;
mod routes;
mod types;
mod util;

use actix_web::middleware as actix_middlewares;
use actix_web::{web, App, HttpServer};

use actix_governor::KeyExtractor;
use actix_governor::{Governor, GovernorConfigBuilder};

use app_state::AppState;
use env_logger::Env;
use tracing_actix_web::TracingLogger;
use util::{ApiError, RateLimitError};

/// Main server configuration and startup
/// Configures:
/// - Rate limiting (3 requests/second, burst size 10)
/// - Request compression
/// - CORS
/// - Request tracing
/// - API routes
/// Binds to: 0.0.0.0:8080
#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_state = web::Data::new(AppState::new().await?);

    #[derive(Clone)]
    pub struct RateLimitKey;

    impl KeyExtractor for RateLimitKey {
        type Key = String;
        type KeyExtractionError = RateLimitError;
        fn extract(
            &self,
            req: &actix_web::dev::ServiceRequest,
        ) -> Result<Self::Key, Self::KeyExtractionError> {
            let head = req.head();
            match head.headers().get("Authorization") {
                Some(data) => return Ok(data.to_str().unwrap().to_string()),
                None => return Ok("Demo String for Testing".to_string()),
            };
        }
    }

    impl RateLimitKey {
        fn new() -> Self {
            RateLimitKey
        }
    }

    let governor_conf = GovernorConfigBuilder::default()
        .key_extractor(RateLimitKey::new())
        .seconds_per_request(3)
        .burst_size(10)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(actix_middlewares::Compress::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(middlewares::cors::cors())
            .app_data(app_state.clone())
            .configure(routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await?;

    Ok(())
}
