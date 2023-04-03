use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::Category;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostOverview {
    pub id: i64,
    pub category: Option<Category>,
    pub title: String,
    pub digest: String,
    pub sketch: String,
    pub tags: String,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    pub create_at: String,
    pub status_sign: StatusSign,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostDetail {
    pub id: i64,
    pub category: Option<Category>,
    pub title: String,
    pub digest: String,
    pub sketch: String,
    pub markdown: String,
    pub html: String,
    pub tags: String,
    pub secret: String,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    pub create_at: String,
    pub status_sign: StatusSign,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusSign {
    DRAFT,
    RELEASE,
    DISCARD,
}

impl StatusSign {
    pub fn toi16(&self) -> i16 {
        match self {
            StatusSign::DRAFT => 0,
            StatusSign::RELEASE => 1,
            StatusSign::DISCARD => -1,
        }
    }
}
