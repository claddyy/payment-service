use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub from_account_id: i32,
    pub to_account_id: i32,
    pub amount: f64,
}

/// Response for a single transaction
#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: i32,
    pub from_account_id: i32,
    pub to_account_id: i32,
    pub amount: f64,
    pub created_at: NaiveDate,
}

impl TransactionResponse {
    pub fn new(
        id: i32,
        from_account_id: i32,
        to_account_id: i32,
        amount: f64,
        created_at: NaiveDate,
    ) -> Self {
        Self {
            id,
            from_account_id,
            to_account_id,
            amount,
            created_at,
        }
    }
}

/// Response for listing multiple transactions
#[derive(Debug, Serialize)]
pub struct ListTransactionsResponse {
    pub transactions: Vec<TransactionResponse>,
}
