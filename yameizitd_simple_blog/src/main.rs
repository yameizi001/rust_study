use sqlx::postgres::PgPoolOptions;

mod config;
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
    let records: Vec<model::Category> = sqlx::query_as!(
        model::Category,
        "select id, name, num from simple_blog_category"
    )
    .fetch_all(&pool)
    .await?;
    tracing::debug!("Records of simple_blog_category:\n{:#?}", records);
    Ok(())
}
