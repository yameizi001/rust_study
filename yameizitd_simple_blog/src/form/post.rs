use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftForm {
    pub category_id: i64,
    pub title: String,
    pub digest: Option<String>,
    pub sketch: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub tags: Option<String>,
    pub secret: Option<String>,
}
