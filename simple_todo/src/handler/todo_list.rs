use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::{
    db,
    model::{CreateTodoList, ToDoList, ToDoListID, UpdateTodoList},
    response, AppState,
};

use super::{get_client, log_error};

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Json(list): Json<CreateTodoList>,
) -> crate::Result<Json<response::Response<ToDoListID>>> {
    let handler_name = "todo_list::add";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_list::create(&client, list)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn find_all(
    Extension(state): Extension<Arc<AppState>>,
) -> crate::Result<Json<response::Response<Vec<ToDoList>>>> {
    let handler_name = "todo_list::find_all";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_list::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn find_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> crate::Result<Json<response::Response<ToDoList>>> {
    let handler_name = "todo_list::find_by_id";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_list::find(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Json(list): Json<UpdateTodoList>,
) -> crate::Result<Json<response::Response<bool>>> {
    let handler_name = "todo_list::update";
    let client = get_client(&state, handler_name).await?;
    let result = db::todo_list::update(&client, list)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}

pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> crate::Result<Json<response::Response<bool>>> {
    let handler_name = "todo_list::delete";
    let mut client = get_client(&state, handler_name).await?;
    let result = db::todo_list::delete(&mut client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(response::Response::ok(result)))
}
