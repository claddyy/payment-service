use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use common::error::thiserror;
use db::util::DBError;
use std::io::Error;

use crate::types::ErrorResponse;

/// Main error types for API operations
/// Handles:
/// - Database errors (500)
/// - IO errors (500)
/// - Authentication errors (401)
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("An Database Error has occurred. Please try again later")]
    DBError(#[from] DBError),

    #[error("An IO Error has occurred. Please try again later")]
    IoError(#[from] Error),

    #[error("An Authentication error has occurred please try again later")]
    AuthError(#[from] AuthError),

    #[error("The account has not enough balance to proceed the transaction")]
    NotEnoughBalance,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotEnoughBalance => StatusCode::BAD_REQUEST,
            Self::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match *self {
            _ => {
                let error_response = ErrorResponse::new(
                    "Database".to_string(),
                    self.to_string(),
                    self.status_code().as_u16(),
                );
                return HttpResponse::build(self.status_code()).json(error_response);
            }
        }
    }
}

/// Rate limiting error response
/// Returns 429 Too Many Requests when rate limit is exceeded
#[derive(thiserror::Error, Debug)]
#[error("You are being Rate Limited")]
pub struct RateLimitError;

impl ResponseError for RateLimitError {
    fn status_code(&self) -> StatusCode {
        StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse::new(
            "Rate Limited".to_string(),
            "Too many Requests sended".to_string(),
            429,
        );
        HttpResponse::build(StatusCode::TOO_MANY_REQUESTS).json(error_response)
    }
}

/// Authentication-specific errors
/// Handles:
/// - JWT token errors
/// - Missing/invalid tokens
/// - User not found
/// - Account/Transaction access unauthorized
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Error generating the Authentication Token")]
    TokenError(#[from] jsonwebtoken::errors::Error),

    #[error("Token is not a valid ASCII string")]
    TokenNotValidASCII,

    #[error("You have no token! Please Login!!")]
    TokenNotFound,

    #[error("Bcrypt Hashing Error")]
    BcryptError(#[from] pwhash::error::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("You are unauthorized to see this page")]
    Unauthorized,

    #[error("Account Not found")]
    AccountNotFound,

    #[error("Transaction Not found")]
    TransactionNotFound,
}
