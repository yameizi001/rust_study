use askama::Template;
use axum::response::Html;
use deadpool_postgres::Client;

use crate::{error::AppError, AppState, HandlerHtmlResult, Result};

pub mod url;

pub use url::*;

pub async fn get_client(state: AppState, handler_name: &str) -> Result<Client> {
    let client = state.pool.get().await.map_err(|err| {
        tracing::error!("Handler[{}] get client failed: {:?}", handler_name, err);
        AppError::from(err)
    })?;
    Ok(client)
}

fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("Handler[{}] failed: {:?}", handler_name, err);
        err
    })
}

fn render<T: Template>(tmpl: T) -> HandlerHtmlResult {
    let html = tmpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}
