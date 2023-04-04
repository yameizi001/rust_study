use serde::{Deserialize, Serialize};

use crate::model::StatusSign;

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftForm {
    pub category_id: Option<i64>,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub tags: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseForm {
    pub category_id: Option<i64>,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub tags: Option<String>,
    pub secret: Option<String>,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateForm {
    pub id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub tags: Option<String>,
    pub secret: Option<String>,
    pub status_sign: StatusSign,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryForm {
    pub id: Option<i64>,
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub tags: Option<String>,
    pub status_sign: Option<String>,
    pub is_private: Option<bool>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
