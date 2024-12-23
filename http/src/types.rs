use serde::Serialize;

/// Standard error response structure for all API endpoints
#[derive(Serialize)]
pub struct ErrorResponse {
    pub cause: String,
    pub description: String,
    pub status_code: u16,
}

impl ErrorResponse {
    pub fn new(cause: String, description: String, status_code: u16) -> Self {
        ErrorResponse {
            cause,
            description,
            status_code,
        }
    }
}
