use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct ToDoList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct ToDoListID {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct ToDoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool,
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct ToDoItemID {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub list_id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoItem {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CheckTodoItem {
    pub id: i32,
}