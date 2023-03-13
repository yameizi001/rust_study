use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::tmpl::MsgTemplate;

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

#[derive(Deserialize)]
pub struct MsgArgs {
    pub ok: Option<String>,
    pub err: Option<String>,
    pub target: Option<String>,
}

impl ToString for MsgArgs {
    fn to_string(&self) -> String {
        let mut r: Vec<String> = vec![];
        if let Some(target) = self.target.clone() {
            r.push(format!("target={}", target));
        }
        if let Some(msg) = self.ok.clone() {
            r.push(format!("ok={}", msg));
        }
        if let Some(msg) = self.err.clone() {
            r.push(format!("err={}", msg));
        }
        r.join("&")
    }
}

impl Into<MsgTemplate> for MsgArgs {
    fn into(self) -> MsgTemplate {
        let mut tmpl = MsgTemplate::default();
        tmpl.target_url = self.target.clone();
        match self {
            MsgArgs { ok: Some(msg), .. } => {
                tmpl.is_ok = true;
                tmpl.msg = msg.clone();
            }
            MsgArgs { err: Some(msg), .. } => {
                tmpl.is_ok = false;
                tmpl.msg = msg.clone();
            }
            _ => {}
        }
        tmpl
    }
}
