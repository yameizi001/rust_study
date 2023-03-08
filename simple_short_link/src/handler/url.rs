use crate::{tmpl::IndexTemplate, HandlerHtmlResult};

use super::{log_error, render};

pub async fn index() -> HandlerHtmlResult {
    let handler_name = "url::index";
    let tmpl = IndexTemplate {};
    let html = render(tmpl).map_err(log_error(handler_name.to_string()))?;
    Ok(html)
}
