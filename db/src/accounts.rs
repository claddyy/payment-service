use crate::db_conn::DB;
use crate::util::DBError;
use entity::accounts::{ActiveModel, Model};
use entity::prelude::Accounts;
use sea_orm::{ColumnTrait, QueryFilter, QueryOrder};
use sea_orm::{EntityTrait, Set};

use std::sync::Arc;

use num_traits::cast::ToPrimitive;
use sea_orm::entity::prelude::Decimal;

pub struct AccountsImpl {
    db: Arc<DB>,
}

impl AccountsImpl {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_account(&self, balance: Option<f64>, user_id: i32) -> Result<i32, DBError> {
        let db = self.db.get()?;
        let bal_to_be_updated: f64;
        if let Some(bal) = balance {
            bal_to_be_updated = bal;
        } else {
            bal_to_be_updated = 0f64;
        }
        let account = ActiveModel {
            user_id: Set(user_id),
            balance: Set(Decimal::from_f64_retain(bal_to_be_updated).unwrap()),
            ..Default::default()
        };
        let result = Accounts::insert(account).exec(db).await?;
        Ok(result.last_insert_id)
    }

    pub async fn find_account(&self, id: i32) -> Result<Option<Model>, DBError> {
        let db = self.db.get()?;
        let acc = Accounts::find_by_id(id).one(db).await?;
        Ok(acc)
    }

    pub async fn list_user_accounts(&self, user_id: i32) -> Result<Vec<Model>, DBError> {
        let db = self.db.get()?;

        let accounts = Accounts::find()
            .filter(entity::accounts::Column::UserId.eq(user_id))
            .order_by_asc(entity::accounts::Column::Id)
            .all(db)
            .await?;

        Ok(accounts)
    }

    pub async fn add_money(&self, id: i32, amount: f64) -> Result<(), DBError> {
        let db = self.db.get()?;
        let acc_from_db = self.find_account(id).await?;
        if let Some(account) = acc_from_db {
            let balance = account.balance.to_f64().unwrap();
            let mut active_model: ActiveModel = account.into();
            let added_money = balance + amount;
            let decimal = Decimal::from_f64_retain(added_money).unwrap();
            active_model.balance = Set(decimal);
            Accounts::update(active_model).exec(db).await?;
        }
        Ok(())
    }

    pub async fn deduct_money(&self, id: i32, amount: f64) -> Result<(), DBError> {
        let db = self.db.get()?;
        let acc_from_db = self.find_account(id).await?;
        if let Some(account) = acc_from_db {
            let balance = account.balance.to_f64().unwrap();
            let mut active_model: ActiveModel = account.into();
            let deducted_money = balance - amount;
            let decimal = Decimal::from_f64_retain(deducted_money).unwrap();
            active_model.balance = Set(decimal);
            Accounts::update(active_model).exec(db).await?;
        }
        Ok(())
    }
}
