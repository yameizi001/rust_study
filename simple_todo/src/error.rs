use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::response;

#[derive(Debug, Serialize)]
pub enum AppErrorType {
    DbType,
    NotFound,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub msg: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn code(&self) -> i32 {
        match &self.error_type {
            AppErrorType::DbType => 1,
            AppErrorType::NotFound => 2,
        }
    }

    pub fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        AppError {
            msg: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }

    pub fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        AppError {
            msg: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }

    pub fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbType)
    }

    pub fn not_found() -> Self {
        Self::from_str("Not found", AppErrorType::NotFound)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = match self.msg {
            Some(msg) => msg,
            None => "Some error occurred".to_string(),
        };
        let response = response::Response::<()>::err(code, msg);
        Json(response).into_response()
    }
}
