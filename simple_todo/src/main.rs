use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put, Router},
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
    // init log
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "simple_todo=debug");
    }
    tracing_subscriber::fmt::init();

    // load .env file
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Config error");
    let pool = cfg
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("Init database pool failed");

    // init state
    let state = Arc::new(AppState { pool });

    // init route
    let app = Router::new()
        .route("/", get(handler::usage::usage))
        .route("/todo_list", post(handler::todo_list::add))
        .route("/todo_list", get(handler::todo_list::find_all))
        .route("/todo_list/:id", get(handler::todo_list::find_by_id))
        .route("/todo_list", put(handler::todo_list::update))
        .route("/todo_list/:id", delete(handler::todo_list::delete))
        .route("/todo_item", post(handler::todo_item::add))
        .route("/todo_item", get(handler::todo_item::find_all))
        .route("/todo_item/:id", get(handler::todo_item::find_by_id))
        .route(
            "/todo_item/list/:id",
            get(handler::todo_item::find_by_list_id),
        )
        .route("/todo_item", put(handler::todo_item::update))
        .route("/todo_item/:id", put(handler::todo_item::check))
        .route("/todo_item/:id", delete(handler::todo_item::delete))
        .layer(Extension(state));

    // listen
    tracing::info!("Server bind on: {}", &cfg.web.addr);
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}
