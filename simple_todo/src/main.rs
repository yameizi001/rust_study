use axum::routing::{get, Router};

mod config;
mod handler;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Config error");

    // init route
    let app = Router::new().route("/", get(handler::usage));

    // listen
    println!("bind: {}", &cfg.web.addr);
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
