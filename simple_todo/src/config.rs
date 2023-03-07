use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub web: WebConfig,
    pub pg: deadpool_postgres::Config,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebConfig {
    pub addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            // load config from .env by dotenv
            .add_source(config::Environment::default())
            .build()?;
        cfg.try_deserialize()
    }
}
