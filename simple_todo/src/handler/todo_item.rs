use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::{
    db,
    model::{CreateTodoItem, ToDoItem, ToDoItemID, UpdateTodoItem},
    response::{self, Response},
    AppState, Result,
};

use super::{get_client, log_error};

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Json(item): Json<CreateTodoItem>,
) -> Result<Json<Response<ToDoItemID>>> {
    let handler_name = "todo_item::add";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::create(&client, item)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn find_all(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Response<Vec<ToDoItem>>>> {
    let handler_name = "todo_item::find_all";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn find_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Response<ToDoItem>>> {
    let handler_name = "todo_item::find_by_id";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::find(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn find_by_list_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Response<Vec<ToDoItem>>>> {
    let handler_name = "todo_item::find_by_list_id";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::link(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Json(item): Json<UpdateTodoItem>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_item::update";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::update(&client, item)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn check(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_item::check";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::check(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_item::delete";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_item::delete(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}
