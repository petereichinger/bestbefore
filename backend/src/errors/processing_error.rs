use std::sync::PoisonError;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use sqlx::Error;

pub enum ProcessingError {
    SqlxError(sqlx::Error),
    PoisonError,
}

impl From<Error> for ProcessingError {
    fn from(sqlx_error: Error) -> Self {
        Self::SqlxError(sqlx_error)
    }
}

impl<T> From<PoisonError<T>> for ProcessingError {
    fn from(_: PoisonError<T>) -> Self {
        ProcessingError::PoisonError
    }
}

impl IntoResponse for ProcessingError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ProcessingError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProcessingError::PoisonError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = match &self {
            ProcessingError::SqlxError(e) => format!("Sql error occurred {}", e.to_string()),
            ProcessingError::PoisonError => format!("Concurrency failure :("),
        };

        (status, Json(json!({ "error": body }))).into_response()
    }
}
