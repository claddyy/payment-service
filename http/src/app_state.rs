use crate::constants;
use db::db_client::DbClient;
use db::util::DBError;
use tracing::info;

/// Global application state shared across all requests, containing Application name and Database connection pool.
pub struct AppState {
    name: String,
    db: DbClient,
}

impl AppState {
    pub async fn new() -> Result<Self, DBError> {
        let app_state = AppState {
            name: constants::APP_NAME.to_string(),
            db: DbClient::new().await?,
        };
        info!("Creating new Global App State for {}", app_state.name());
        Ok(app_state)
    }

    pub fn db(&self) -> &DbClient {
        &self.db
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
