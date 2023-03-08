use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Extension, Router,
};
use handler::index;

mod config;
mod core;
mod db;
mod error;
mod handler;
mod model;
mod tmpl;

pub type Result<T> = std::result::Result<T, error::AppError>;
pub type HandlerResult<T> = self::Result<T>;
pub type RedirectResponse = (StatusCode, HeaderMap, ());
pub type HandlerRedirectResult = self::HandlerResult<RedirectResponse>;
pub type HtmlResponse = Html<String>;
pub type HandlerHtmlResult = HandlerResult<HtmlResponse>;

#[tokio::main]
async fn main() {
    // init log
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "simple_short_link=debug");
    }
    tracing_subscriber::fmt::init();

    // init config
    dotenv::dotenv().ok();
    let config = config::Config::from_env().expect("Failed to load config from environment");

    // init state
    let pool = config
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("Failed to create postgres pool");
    let state = Arc::new(AppState { pool });

    // init route
    let app = Router::new().route("/", get(index)).layer(Extension(state));

    // start server
    axum::Server::bind(&config.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}
