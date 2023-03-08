use std::fmt::Display;

use crate::tmpl::MsgTemplate;
use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFound,
    TemplateError,
}

#[derive(Debug)]
pub struct AppError {
    pub msg: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn code(&self) -> i32 {
        match &self.error_type {
            AppErrorType::DbError => 1,
            AppErrorType::NotFound => 2,
            AppErrorType::TemplateError => 3,
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
        Self::from_err(err, AppErrorType::DbError)
    }

    pub fn tmpl_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::TemplateError)
    }

    pub fn not_found() -> Self {
        Self::from_str("Not found", AppErrorType::NotFound)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_error(err)
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_error(err)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::tmpl_error(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = self.msg.unwrap_or("Some error occurred".to_string());
        let tmpl = MsgTemplate {
            is_ok: false,
            msg: msg.clone(),
            target_url: None,
        };
        let html = tmpl.render().unwrap_or(msg);
        Html(html).into_response()
    }
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
