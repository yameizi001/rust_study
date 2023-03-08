use tokio_postgres::Client;

use crate::{
    model::{CreateTodoItem, ToDoItem, ToDoItemID, UpdateTodoItem},
    Result,
};

use super::{execute, query, query_one};

pub async fn create(client: &Client, item: CreateTodoItem) -> Result<ToDoItemID> {
    let result = query_one(
        client,
        "insert into todo_item (list_id, title, checked) values ($1, $2, false) returning id",
        &[&item.list_id, &item.title],
    )
    .await?;
    Ok(result)
}

pub async fn all(client: &Client) -> Result<Vec<ToDoItem>> {
    let result = query(client, "select * from todo_item", &[]).await?;
    Ok(result)
}

pub async fn find(client: &Client, id: i32) -> Result<ToDoItem> {
    let result = query_one(
        client,
        "select * from todo_item where id = $1 order by id desc",
        &[&id],
    )
    .await?;
    Ok(result)
}

pub async fn link(client: &Client, id: i32) -> Result<Vec<ToDoItem>> {
    let result = query(client, "select * from todo_item where list_id = $1", &[&id]).await?;
    Ok(result)
}

pub async fn update(client: &Client, item: UpdateTodoItem) -> Result<bool> {
    let result = execute(
        client,
        "update todo_item set title = $2 where id = $1",
        &[&item.id, &item.title],
    )
    .await?;
    Ok(result > 0)
}

pub async fn check(client: &Client, id: i32) -> Result<bool> {
    let result = execute(
        client,
        "update todo_item set checked = true where id = $1",
        &[&id],
    )
    .await?;
    Ok(result > 0)
}

pub async fn delete(client: &Client, id: i32) -> Result<bool> {
    let result = execute(client, "delete from todo_item where id = $1", &[&id]).await?;
    Ok(result > 0)
}
