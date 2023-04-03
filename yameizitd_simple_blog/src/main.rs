use sqlx::postgres::PgPoolOptions;

use crate::{db::post, form::DraftForm};

mod config;
mod db;
mod error;
mod form;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init tracing
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "yameizitd_simple_blog=debug");
    }
    tracing_subscriber::fmt::init();

    // init config
    dotenv::dotenv().ok();
    let app_config = config::AppConfig::from_env().expect("Failed to load config");
    tracing::debug!("App config as follows:\n{:#?}", app_config);

    // init postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(app_config.postgres.pool.max_size)
        .connect(&app_config.postgres.build_connection())
        .await?;
    let draft = DraftForm {
        category_id: None,
        title: "test draft".to_string(),
        digest: Some("description".to_string()),
        sketch: None,
        markdown: None,
        html: None,
        tags: None,
        secret: None,
    };
    let id = post::insert_draft(&pool, draft).await?;
    tracing::debug!("Insert draft, id: {}", id);
    tracing::debug!("Init postgres pool successfully");
    Ok(())
}
