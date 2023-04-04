use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::error::DbError;

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
    #[sqlx(default)]
    #[sqlx(flatten)]
    pub category: Option<Category>,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub tags: Option<String>,
    pub secret: Option<String>,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    pub create_at: String,
    #[sqlx(try_from = "i16")]
    pub status_sign: StatusSign,
    pub is_private: bool,
}

impl PostDetail {
    pub fn from_row(row: PgRow) -> Self {
        PostDetail {
            id: row.get("id"),
            category: if row.try_get::<i64, _>("category_id").is_ok() {
                Some(Category {
                    id: row.get("category_id"),
                    name: row.get("name"),
                    num: row.get("num"),
                })
            } else {
                None
            },
            title: row.get("title"),
            digest: if row.try_get::<String, _>("digest").is_ok() {
                Some(row.get("digest"))
            } else {
                None
            },
            sketch: if row.try_get::<String, _>("sketch").is_ok() {
                Some(row.get("sketch"))
            } else {
                None
            },
            markdown: if row.try_get::<String, _>("markdown").is_ok() {
                Some(row.get("markdown"))
            } else {
                None
            },
            html: if row.try_get::<String, _>("html").is_ok() {
                Some(row.get("html"))
            } else {
                None
            },
            tags: if row.try_get::<String, _>("tags").is_ok() {
                Some(row.get("tags"))
            } else {
                None
            },
            secret: if row.try_get::<String, _>("secret").is_ok() {
                Some(row.get("secret"))
            } else {
                None
            },
            views: row.get("views"),
            likes: row.get("likes"),
            comments: row.get("comments"),
            create_at: row.get("create_at"),
            status_sign: row.get::<i16, _>("status_sign").try_into().unwrap(),
            is_private: row.get("is_private"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    #[sqlx(rename = "category_id")]
    pub id: i64,
    pub name: String,
    pub num: i64,
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
