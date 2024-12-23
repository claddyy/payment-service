#![allow(deprecated)]
#![allow(unused)]

use chrono::Local;
use entity::prelude::User;
use entity::user::{ActiveModel, Model};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{EntityTrait, Set};
use std::sync::Arc;

use crate::db_conn::DB;
use crate::util::DBError;

pub struct UserImpl {
    db: Arc<DB>,
}

impl UserImpl {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: String, password: String) -> Result<i32, DBError> {
        let db = self.db.get()?;
        let user = ActiveModel {
            username: Set(username),
            // TODO: hash this password
            password: Set(password),
            created_at: Set(Local::today().naive_local()),
            ..Default::default()
        };
        let result = User::insert(user).exec(db).await?;

        Ok(result.last_insert_id)
    }

    pub async fn find_user(&self, id: i32) -> Result<Option<Model>, DBError> {
        let db = self.db.get()?;
        let user = User::find_by_id(id).one(db).await?;
        Ok(user)
    }

    pub async fn find_user_by_username(&self, username: String) -> Result<Option<Model>, DBError> {
        let db = self.db.get()?;
        let user = User::find()
            .filter(entity::user::Column::Username.eq(username))
            .one(db)
            .await?;
        Ok(user)
    }
}
