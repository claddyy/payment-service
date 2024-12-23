use crate::app_state::AppState;
use crate::features::transactions::transaction_types::{
    CreateTransactionRequest, ListTransactionsResponse, TransactionResponse,
};
use crate::middlewares::auth::JWTClaim;
use crate::util::ApiError;
use crate::util::AuthError;
use actix_web::{get, post, web, Responder};
use num_traits::cast::ToPrimitive;

type State = web::Data<AppState>;

/// Create a new transaction between accounts
/// Endpoint: POST /api/transaction/create
/// Request Body: {
///     "from_account_id": integer,
///     "to_account_id": integer,
///     "amount": float
/// }
/// Response Body: {
///     "id": integer,
///     "from_account_id": integer,
///     "to_account_id": integer,
///     "amount": float,
///     "created_at": string
/// }
/// Requires authentication.
/// Returns error if user doesn't own an account
#[post("/create")]
async fn create_transaction(
    state: State,
    claim: JWTClaim,
    request: web::Json<CreateTransactionRequest>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();
    let amount = request.amount;

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let from_account = db
        .account
        .find_account(request.from_account_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::AccountNotFound))?;

    let to_account = db
        .account
        .find_account(request.to_account_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::AccountNotFound))?;

    if from_account.user_id != user_id {
        return Err(ApiError::AuthError(AuthError::Unauthorized));
    }

    if from_account.balance.to_f64().unwrap() < request.amount {
        return Err(ApiError::NotEnoughBalance);
    }

    let transaction_id = db
        .transaction
        .create_transaction(
            request.from_account_id,
            request.to_account_id,
            request.amount,
        )
        .await?;

    db.account.deduct_money(from_account.id, amount).await?;
    db.account.add_money(to_account.id, amount).await?;

    let transaction = db
        .transaction
        .find_transaction(transaction_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::TransactionNotFound))?;

    let response = TransactionResponse::new(
        transaction.id,
        transaction.from_account_id,
        transaction.to_account_id,
        transaction.amount.to_f64().unwrap(),
        transaction.created_at,
    );

    Ok(web::Json(response))
}

/// List all transactions involving user's accounts
/// Endpoint: GET /api/transaction/user/tx
/// Response Body: {
///     "transactions": [
///         {
///             "id": integer,
///             "from_account_id": integer,
///             "to_account_id": integer,
///             "amount": float,
///             "created_at": string
///         }
///     ]
/// }
/// Requires authentication. Returns empty list if user has no accounts
#[get("/user/tx")]
async fn list_user_transactions(state: State, claim: JWTClaim) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let user_accounts = db.account.list_user_accounts(user_id).await?;
    let account_ids: Vec<i32> = user_accounts
        .into_iter()
        .map(|account| account.id)
        .collect();

    if account_ids.is_empty() {
        return Ok(web::Json(ListTransactionsResponse {
            transactions: vec![],
        }));
    }

    let transactions = db
        .transaction
        .list_transactions_for_accounts(&account_ids)
        .await?;

    let transaction_responses: Vec<TransactionResponse> = transactions
        .into_iter()
        .map(|t| {
            TransactionResponse::new(
                t.id,
                t.from_account_id,
                t.to_account_id,
                t.amount.to_f64().unwrap(),
                t.created_at,
            )
        })
        .collect();

    Ok(web::Json(ListTransactionsResponse {
        transactions: transaction_responses,
    }))
}

/// Get details of a specific transaction
/// Endpoint: GET /api/transaction/{transaction_id}
/// Path Parameters: transaction_id (integer)
/// Response Body: {
///     "id": integer,
///     "from_account_id": integer,
///     "to_account_id": integer,
///     "amount": float,
///     "created_at": string
/// }
/// Requires authentication. Returns error if user doesn't own either account involved
#[get("/{transaction_id}")]
async fn get_transaction(
    state: State,
    claim: JWTClaim,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let user_id = claim.id();
    let transaction_id = path.into_inner();

    let _user = db
        .user
        .find_user(user_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::UserNotFound))?;

    let transaction = db
        .transaction
        .find_transaction(transaction_id)
        .await?
        .ok_or(ApiError::AuthError(AuthError::TransactionNotFound))?;

    let from_account = db.account.find_account(transaction.from_account_id).await?;
    let to_account = db.account.find_account(transaction.to_account_id).await?;

    let is_authorized = from_account
        .as_ref()
        .map_or(false, |acc| acc.user_id == user_id)
        || to_account
            .as_ref()
            .map_or(false, |acc| acc.user_id == user_id);

    if !is_authorized {
        return Err(ApiError::AuthError(AuthError::Unauthorized));
    }

    let response = TransactionResponse::new(
        transaction.id,
        transaction.from_account_id,
        transaction.to_account_id,
        transaction.amount.to_f64().unwrap(),
        transaction.created_at,
    );

    Ok(web::Json(response))
}
