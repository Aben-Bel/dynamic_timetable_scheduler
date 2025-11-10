use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub enum ApiError {
    ItemNotFound,
    MemberNotFound,
    InvalidInput,
    UnexpectedError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::ItemNotFound => (StatusCode::NOT_FOUND, "Item not found"),
            ApiError::MemberNotFound => (StatusCode::NOT_FOUND, "Member not found"),
            ApiError::InvalidInput => (StatusCode::BAD_REQUEST, "Invalid input"),
            ApiError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
