use askama::Template;

use crate::model::Url;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "msg.html")]
pub struct MsgTemplate {
    pub is_ok: bool,
    pub msg: String,
    pub target_url: Option<String>,
}

#[derive(Template)]
#[template(path = "rank.html")]
pub struct RankTemplate {
    pub urls: Vec<Url>,
}
