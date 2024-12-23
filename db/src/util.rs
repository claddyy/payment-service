use std::env::VarError;

use common::error::thiserror;
use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("Could not connect to the Database")]
    DBErr(#[from] DbErr),

    #[error("The Environment Variable: DATABASE_URL must be set")]
    VarError(#[from] VarError),
}
