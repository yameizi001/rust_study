use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;

use crate::{
    error::AppError,
    model::{CreateTodoList, ToDoList, ToDoListID, UpdateTodoList},
    Result,
};

use super::{execute, query, query_one};

// create todo list
pub async fn create(client: &Client, list: CreateTodoList) -> Result<ToDoListID> {
    let result = query_one(
        client,
        "insert into todo_list (title) values ($1) returning id",
        &[&list.title],
    )
    .await?;
    Ok(result)
}

// get all todo list
pub async fn all(client: &Client) -> Result<Vec<ToDoList>> {
    let result = query(client, "select * from todo_list order by id desc", &[]).await?;
    Ok(result)
}

// get todo list by id
pub async fn find(client: &Client, id: i32) -> Result<ToDoList> {
    let result = query_one(
        client,
        "select * from todo_list where id = $1 order by id desc",
        &[&id],
    )
    .await?;
    Ok(result)
}

// update todo list by id
pub async fn update(client: &Client, list: UpdateTodoList) -> Result<bool> {
    let result = execute(
        client,
        "update todo_list set title = $2 where id = $1",
        &[&list.id, &list.title],
    )
    .await?;
    Ok(result > 0)
}

// delete todo list by id(cascade delete todo item)
pub async fn delete(client: &mut Client, id: i32) -> Result<bool> {
    // init transaction
    let tx = client.transaction().await.map_err(AppError::from)?;
    // delete todo list by id
    let result = execute(&tx, "delete from todo_list where id = $1", &[&id]).await;
    // rollback when some error happened
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    }
    // cascade delete todo item
    let result = execute(&tx, "delete from todo_item where id = $1", &[&id]).await;
    // rollback when some error happened
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}
