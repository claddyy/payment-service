use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

impl AuthResponse {
    pub fn new(token: String) -> AuthResponse {
        AuthResponse { token }
    }
}

/// Request body for user registration
#[derive(Deserialize)]
pub struct UserRegisterRequest {
    pub username: String,
    pub password: String,
}

impl UserRegisterRequest {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

/// Request body for user login
#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

impl UserLoginRequest {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

/// Response for successful registration
#[derive(Serialize)]
pub struct UserRegisterResponse {
    user_id: i32,
    username: String,
    token: String,
}

impl UserRegisterResponse {
    pub fn new(user_id: i32, username: String, token: String) -> Self {
        Self {
            username,
            user_id,
            token,
        }
    }
}

/// Response for successful login
#[derive(Serialize)]
pub struct UserLoginResponse {
    user_id: i32,
    username: String,
    token: String,
}

impl UserLoginResponse {
    pub fn new(user_id: i32, username: String, token: String) -> Self {
        Self {
            username,
            user_id,
            token,
        }
    }
}
