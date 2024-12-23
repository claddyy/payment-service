#![allow(deprecated)]

use std::sync::Arc;

use chrono::Local;
use entity::prelude::Transactions;
use entity::transactions::{ActiveModel, Model};

use crate::db_conn::DB;
use crate::util::DBError;
use sea_orm::prelude::Decimal;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::{ColumnTrait, Condition, EntityTrait, Set};

pub struct TransactionImpl {
    db: Arc<DB>,
}

impl TransactionImpl {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_transaction(
        &self,
        from: i32,
        to: i32,
        amount: f64,
    ) -> Result<i32, DBError> {
        let db = self.db.get()?;
        let transaction = ActiveModel {
            from_account_id: Set(from),
            to_account_id: Set(to),
            amount: Set(Decimal::from_f64_retain(amount).unwrap()),
            created_at: Set(Local::today().naive_local()),
            ..Default::default()
        };
        let result = Transactions::insert(transaction).exec(db).await?;
        Ok(result.last_insert_id)
    }

    pub async fn find_transaction(&self, id: i32) -> Result<Option<Model>, DBError> {
        let db = self.db.get()?;
        let transaction = Transactions::find_by_id(id).one(db).await?;
        Ok(transaction)
    }

    pub async fn list_transactions_for_accounts(
        &self,
        account_ids: &[i32],
    ) -> Result<Vec<Model>, DBError> {
        let db = self.db.get()?;
        let transactions = Transactions::find()
            .filter(
                Condition::any()
                    .add(entity::transactions::Column::FromAccountId.is_in(account_ids.to_vec()))
                    .add(entity::transactions::Column::ToAccountId.is_in(account_ids.to_vec())),
            )
            .order_by_desc(entity::transactions::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(transactions)
    }
}
