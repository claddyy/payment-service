use actix_web::{get, post, web, Responder};
use chrono::Local;

use crate::{
    app_state::AppState,
    middlewares::auth::JWTClaim,
    util::{ApiError, AuthError},
};

use super::account_types::{
    AccountResponse, CreateAccountRequest, CreateAccountResponse, GetBalanceResponse,
    ListAccountsResponse,
};

use num_traits::cast::ToPrimitive;

type State = web::Data<AppState>;

/// Create a new account for the authenticated user
/// Endpoint: POST /api/account/create
/// Request Body: {
///     "initial_balance": float (optional)
/// }
/// Response Body: {
///     "account_id": integer,
///     "user_id": integer,
///     "balance": float,
///     "created_at": string
/// }
/// Requires authentication. Initial balance defaults to 0 if not provided
#[post("/create")]
async fn create_account(
    state: State,
    claim: JWTClaim,
    request: web::Json<CreateAccountRequest>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let account_id = db
        .account
        .create_account(request.initial_balance, user_id)
        .await?;

    let account = db
        .account
        .find_account(account_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::AccountNotFound))?;

    let response = CreateAccountResponse::new(
        account_id,
        user_id,
        account.balance.to_f64().unwrap(),
        Local::now().date_naive().to_string(),
    );

    Ok(web::Json(response))
}

/// Get details of a specific account
/// Endpoint: GET /api/account/{account_id}
/// Path Parameters: account_id (integer)
/// Response Body: {
///     "id": integer,
///     "user_id": integer,
///     "balance": float,
///     "created_at": string
/// }
/// Requires authentication. Returns error if account doesn't belong to user
#[get("/{account_id}")]
async fn get_account(
    state: State,
    claim: JWTClaim,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();
    let account_id = path.into_inner();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let account = db
        .account
        .find_account(account_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::AccountNotFound))?;

    if account.user_id != user_id {
        return Err(ApiError::AuthError(AuthError::Unauthorized));
    }

    let response = AccountResponse::new(
        account.id,
        account.user_id,
        account.balance.to_f64().unwrap(),
        Local::now().date_naive().to_string(),
    );

    Ok(web::Json(response))
}

/// List all accounts owned by the authenticated user
/// Endpoint: GET /api/account/list/account
/// Response Body: {
///     "accounts": [
///         {
///             "id": integer,
///             "user_id": integer,
///             "balance": float,
///             "created_at": string
///         }
///     ]
/// }
/// Requires authentication
#[get("/list/account")]
async fn list_accounts(state: State, claim: JWTClaim) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let accounts = db.account.list_user_accounts(user_id).await?;

    let account_responses: Vec<AccountResponse> = accounts
        .into_iter()
        .map(|account| {
            AccountResponse::new(
                account.id,
                account.user_id,
                account.balance.to_f64().unwrap(),
                Local::now().date_naive().to_string(),
            )
        })
        .collect();

    Ok(web::Json(ListAccountsResponse {
        accounts: account_responses,
    }))
}

/// Get the current balance of a specific account
/// Endpoint: GET /api/account/{account_id}/balance
/// Path Parameters: account_id (integer)
/// Response Body: {
///     "account_id": integer,
///     "balance": float
/// }
/// Requires authentication.
/// Returns error if an account doesn't belong to user
#[get("/{account_id}/balance")]
async fn get_balance(
    state: State,
    claim: JWTClaim,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();
    let account_id = path.into_inner();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let account = db
        .account
        .find_account(account_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::AccountNotFound))?;

    if account.user_id != user_id {
        return Err(ApiError::AuthError(AuthError::Unauthorized));
    }
    let response = GetBalanceResponse::new(account_id, account.balance.to_f64().unwrap());
    Ok(web::Json(response))
}
