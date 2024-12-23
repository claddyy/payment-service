use actix_web::Error;
use actix_web::FromRequest;
use futures_util::future::err;
use futures_util::future::ok;
use futures_util::future::Ready;
use jsonwebtoken::decode;
use jsonwebtoken::Validation;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::util::ApiError;
use crate::util::AuthError;

pub struct JWTKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JWTKeys {
    pub fn new() -> Self {
        const JWT_SECRET: &str = "dodotakehome";
        let secret = JWT_SECRET.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }

    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding
    }

    pub fn decoding_key(&self) -> &DecodingKey {
        &self.decoding
    }
}

pub static KEYS: Lazy<JWTKeys> = Lazy::new(|| JWTKeys::new());

/// JWT claims structure for authentication
/// Contains:
/// - User ID
/// - Expiration time
/// - Issued at time
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaim {
    user_id: i32,
}
impl JWTClaim {
    pub fn new(user_id: i32) -> Self {
        JWTClaim { user_id }
    }

    pub fn id(&self) -> i32 {
        self.user_id
    }
}

impl FromRequest for JWTClaim {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Some(header) = req.headers().get("Authorization") {
            if let Ok(token) = header.to_str() {
                let key = KEYS.decoding_key();
                let mut validation = Validation::default();
                validation.required_spec_claims.remove("exp");
                validation.validate_exp = false;
                let token_data = decode::<JWTClaim>(token, key, &validation)
                    .map_err(|err| AuthError::TokenError(err));
                match token_data {
                    Ok(data) => ok(data.claims),
                    Err(error) => err(ApiError::AuthError(error).into()),
                }
            } else {
                err(ApiError::AuthError(AuthError::TokenNotValidASCII).into())
            }
        } else {
            err(ApiError::AuthError(AuthError::TokenNotFound).into())
        }
    }
}
