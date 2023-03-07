use std::sync::Arc;

use axum::{
    routing::{get, Router},
    Extension,
};

mod config;
mod db;
mod error;
mod handler;
mod model;
mod response;

type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Config error");
    let pool = cfg
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("Init database pool failed");

    // init route
    let app = Router::new()
        .route("/", get(handler::usage))
        .layer(Extension(Arc::new(AppState { pool })));

    // listen
    println!("bind: {}", &cfg.web.addr);
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}
