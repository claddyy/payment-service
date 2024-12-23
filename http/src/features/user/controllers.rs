use actix_web::{post, web, Responder};

use crate::app_state::AppState;
use crate::middlewares::auth::{JWTClaim, KEYS};
use crate::util::AuthError;
use crate::ApiError;

use jsonwebtoken::encode;
use jsonwebtoken::Header;
use pwhash::bcrypt;

use super::user_types::{
    UserLoginRequest, UserLoginResponse, UserRegisterRequest, UserRegisterResponse,
};

type State = web::Data<AppState>;

/// Register a new user in the system
/// Endpoint: POST/api/user/register
/// Request Body: {
///    "username": "string",
///    "password": "string"
///}
/// Response Body: {
///     "user_id" : integer,
///     "username" : "string",
///     "token" : "string"
/// }
/// Returns an error if registration fails
#[post("/register")]
async fn register(
    state: State,
    request: web::Json<UserRegisterRequest>,
) -> Result<impl Responder, ApiError> {
    let db_client = state.db();
    let username = request.username.clone();
    let password = request.password.clone();
    let password = bcrypt::hash(password).map_err(|e| AuthError::BcryptError(e))?;
    let id = db_client
        .user
        .create_user(username.clone(), password)
        .await
        .map_err(|e| ApiError::DBError(e))?;

    let claim = JWTClaim::new(id);
    let key = KEYS.encoding_key();
    let token =
        encode(&Header::default(), &claim, key).map_err(|err| AuthError::TokenError(err))?;

    let response = UserRegisterResponse::new(id, username, token);

    Ok(web::Json(response))
}

/// Authenticate an existing user
/// Endpoint: POST /api/user/login
/// Request Body: {
///     "username": "string",
///     "password": "string"
/// }
/// Response Body: {
///     "user_id": integer,
///     "username": "string",
///     "token": "string"
/// }
#[post("/login")]
async fn login(
    state: State,
    request: web::Json<UserLoginRequest>,
) -> Result<impl Responder, ApiError> {
    let db = state.db();
    let username = request.username.clone();
    let password = request.password.clone();
    let user = db.user.find_user_by_username(username.clone()).await?;
    if let Some(user) = user {
        let db_password = user.password;
        let user_id = user.id;
        if bcrypt::verify(password, db_password.as_str()) {
            let claim = JWTClaim::new(user_id);
            let key = KEYS.encoding_key();
            let token = encode(&Header::default(), &claim, key)
                .map_err(|err| AuthError::TokenError(err))?;

            let response = UserLoginResponse::new(user_id, username, token);
            Ok(web::Json(response))
        } else {
            Err(ApiError::AuthError(AuthError::Unauthorized))
        }
    } else {
        Err(ApiError::AuthError(AuthError::UserNotFound))
    }
}
