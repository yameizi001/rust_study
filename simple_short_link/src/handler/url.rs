use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    Extension, Form,
};

use crate::{
    core, db,
    error::AppError,
    model::{CreateUrl, MsgArgs},
    tmpl::{IndexTemplate, MsgTemplate, RankTemplate},
    AppState, HandlerHtmlResult, HandlerRedirectResult, RedirectResponse,
};

use super::{get_client, log_error, render};

// goto index
pub async fn index() -> HandlerHtmlResult {
    let handler_name = "url::index";
    let tmpl = IndexTemplate {};
    let html = render(tmpl).map_err(log_error(handler_name.to_string()))?;
    Ok(html)
}

// handle create action
pub async fn create_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(url): Form<CreateUrl>,
) -> HandlerRedirectResult {
    let id = core::short_link(&url.url).map_err(AppError::from)?;
    if (&state).cfg.short_link.in_reserved_words(&id) {
        return Err(AppError::reserved_word(&id));
    }
    let handler_name = "url::create";
    let client = get_client(&state, handler_name).await?;
    let result = db::create(&client, url, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let msg = MsgArgs {
        ok: Some(format!("添加成功, 短网址是：{}", result.id)),
        err: None,
        target: Some("/".to_string()),
    };
    Ok(redirect_with_msg("/msg", Some(&msg)))
}

// goto url
pub async fn goto_url(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> HandlerRedirectResult {
    let handler_name = "url::goto_url";
    let mut client = get_client(&state, handler_name).await?;
    let result = db::goto_url(&mut client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(redirect(result.url.as_str()))
}

// redirect
fn redirect_with_msg(url: &str, args: Option<&MsgArgs>) -> RedirectResponse {
    let url = match args {
        Some(args) => format!("{}?{}", url, args.to_string()),
        None => url.to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, url.as_str().parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

fn redirect(url: &str) -> RedirectResponse {
    redirect_with_msg(url, None)
}

// rank
pub async fn rank(Extension(state): Extension<Arc<AppState>>) -> HandlerHtmlResult {
    let handler_name = "url::rank";
    let client = get_client(&state, handler_name).await?;
    let result = db::rank(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = RankTemplate { urls: result };
    let html = render(tmpl).map_err(log_error(handler_name.to_string()))?;
    Ok(html)
}

// msg
pub async fn msg(Query(args): Query<MsgArgs>) -> HandlerHtmlResult {
    let handler_name = "url::msg";
    let tmpl: MsgTemplate = args.into();
    render(tmpl).map_err(log_error(handler_name.to_string()))
}
