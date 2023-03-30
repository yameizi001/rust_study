use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForm {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateForm {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryForm {
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
    pub id: Option<i64>,
    pub name: Option<String>,
}
