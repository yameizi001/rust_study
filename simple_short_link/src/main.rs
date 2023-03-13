use std::sync::Arc;

use crate::config::Config;
use axum::{
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Extension, Router,
};
use tower_http::services::ServeDir;

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
    let state = Arc::new(AppState {
        pool,
        cfg: config.clone(),
    });

    // init route
    let app = Router::new()
        .route("/", get(handler::index).post(handler::create_action))
        .route("/target/:id", get(handler::goto_url))
        .route("/rank", get(handler::rank))
        .route("/msg", get(handler::msg))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(state));

    // start server
    tracing::info!("Server started on {:?}", &config.web.addr);
    axum::Server::bind(&config.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub cfg: Config,
}
