use std::sync::Arc;

use crate::{
    accounts::AccountsImpl, db_conn::DB, transactions::TransactionImpl, user::UserImpl,
    util::DBError,
};

pub struct DbClient {
    pub user: UserImpl,
    pub account: AccountsImpl,
    pub transaction: TransactionImpl,
}

impl DbClient {
    pub async fn new() -> Result<Self, DBError> {
        let db = Arc::new(DB::new().await?);
        let user_client = UserImpl::new(db.clone());
        let transaction_client = TransactionImpl::new(db.clone());
        let accounts_client = AccountsImpl::new(db.clone());
        let db_client = DbClient {
            user: user_client,
            account: accounts_client,
            transaction: transaction_client,
        };
        Ok(db_client)
    }
}
