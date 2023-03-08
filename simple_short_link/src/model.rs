use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "url")]
pub struct Url {
    pub id: String,
    pub url: String,
    pub email: String,
    pub visit: i32,
    pub is_del: bool,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "url")]
pub struct UrlID {
    pub id: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "url")]
pub struct UrlTarget {
    pub url: String,
}

#[derive(Deserialize)]
pub struct CreateUrl {
    pub url: String,
    pub email: String,
}
#[derive(Deserialize)]
pub struct UpdateUrl {
    pub id: String,
    pub url: String,
    pub email: String,
}
