use sqlx::postgres::PgPoolOptions;

use crate::{
    db::post,
    form::{post::UpdateForm, DraftForm, ReleaseForm},
    model::StatusSign,
};

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
    tracing::debug!("Init postgres pool successfully");
    // let release = ReleaseForm {
    //     category_id: None,
    //     title: "test draft".to_string(),
    //     digest: Some("description".to_string()),
    //     sketch: None,
    //     markdown: None,
    //     html: None,
    //     tags: None,
    //     secret: None,
    //     is_private: false,
    // };
    // let id = post::insert_release(&pool, release).await?;
    let form = UpdateForm {
        id: 5,
        category_id: Some(1),
        title: "update test release".to_string(),
        digest: Some("".to_string()),
        sketch: Some("https://...".to_string()),
        markdown: None,
        html: None,
        tags: Some("tag1,tag2,tag3".to_string()),
        secret: None,
        status_sign: StatusSign::RELEASE,
        is_private: false,
    };
    let updated = post::update(&pool, form).await?;
    tracing::debug!("Update post, id({}): {}", 5, updated);
    Ok(())
}
