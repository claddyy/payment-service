use ::serde::Deserialize;
use serde::Serialize;

/// Request body for creating a new account
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_balance: Option<f64>,
}

/// Response for account creation
#[derive(Serialize)]
pub struct CreateAccountResponse {
    account_id: i32,
    user_id: i32,
    balance: f64,
    created_at: String,
}

impl CreateAccountResponse {
    pub fn new(account_id: i32, user_id: i32, balance: f64, created_at: String) -> Self {
        Self {
            account_id,
            user_id,
            balance,
            created_at,
        }
    }
}

/// Response for account details
#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: i32,
    pub user_id: i32,
    pub balance: f64,
    pub created_at: String,
}

impl AccountResponse {
    pub fn new(id: i32, user_id: i32, balance: f64, created_at: String) -> Self {
        Self {
            id,
            user_id,
            balance,
            created_at,
        }
    }
}

/// Response for listing multiple accounts
#[derive(Debug, Serialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountResponse>,
}

/// Response for account balance query
#[derive(Debug, Serialize)]
pub struct GetBalanceResponse {
    pub account_id: i32,
    pub balance: f64,
}

impl GetBalanceResponse {
    pub fn new(account_id: i32, balance: f64) -> Self {
        Self {
            account_id,
            balance,
        }
    }
}
