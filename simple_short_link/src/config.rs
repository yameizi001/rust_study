use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub pg: deadpool_postgres::Config,
    pub short_link: ShortLinkConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        config.try_deserialize()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortLinkConfig {
    pub reserved_words: String,
    pub domain: String,
}

impl ShortLinkConfig {
    pub fn reserved_words(&self) -> Vec<&str> {
        self.reserved_words.split(',').collect()
    }

    pub fn in_reserved_words(&self, word: &str) -> bool {
        for item in self.reserved_words() {
            if item == word {
                return true;
            }
        }
        false
    }
}
