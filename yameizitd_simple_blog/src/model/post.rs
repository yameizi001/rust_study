use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::DbError;

use super::Category;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostOverview {
    pub id: i64,
    #[sqlx(flatten)]
    pub category: Category,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub tags: Option<String>,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    pub create_at: String,
    #[sqlx(try_from = "i16")]
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

impl TryFrom<i16> for StatusSign {
    type Error = sqlx::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StatusSign::DRAFT),
            1 => Ok(StatusSign::RELEASE),
            -1 => Ok(StatusSign::DISCARD),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "status_sign".to_string(),
                source: Box::new(DbError::DbTypeConvertError {
                    cause: "Failed to convert i16 into StatusSign".to_string(),
                }),
            }),
        }
    }
}
