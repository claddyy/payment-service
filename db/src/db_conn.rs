#![allow(unused_mut)]

use std::{env, time::Duration};

use crate::util::DBError;
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DB {
    db: DatabaseConnection,
}

impl DB {
    pub async fn new() -> Result<Self, DBError> {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")?;

        let mut opts = ConnectOptions::new(db_url);
        opts.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));

        let db = Database::connect(opts).await?;
        Ok(DB { db })
    }

    pub fn get(&self) -> Result<&DatabaseConnection, DBError> {
        Ok(&self.db)
    }

    pub async fn stop(self) -> Result<(), DBError> {
        Ok(self.db.close().await?)
    }

    pub async fn ping(&self) -> Result<bool, DBError> {
        Ok(self.db.ping().await.is_ok())
    }
}
