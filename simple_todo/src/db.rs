mod todo_list {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{
        error::AppError,
        model::{CreateTodoList, ToDoList, ToDoListID, UpdateTodoList},
        Result,
    };

    // create todo list
    pub async fn create(client: &Client, list: CreateTodoList) -> Result<ToDoListID> {
        let stmt = client
            .prepare("insert into todo_list (title) values ($1) returning id")
            .await
            .map_err(AppError::from)?;
        let result = client
            .query(&stmt, &[&list.title])
            .await
            .map_err(AppError::from)?
            .iter()
            .map(|row| ToDoListID::from_row_ref(row).unwrap())
            .collect::<Vec<ToDoListID>>()
            .pop()
            .ok_or(AppError::not_found())?;
        Ok(result)
    }

    // get all todo list
    pub async fn all(client: &Client) -> Result<Vec<ToDoList>> {
        let stmt = client
            .prepare("select * from todo_list order by id desc")
            .await
            .map_err(AppError::from)?;
        let result = client
            .query(&stmt, &[])
            .await
            .map_err(AppError::from)?
            .iter()
            .map(|row| ToDoList::from_row_ref(row).unwrap())
            .collect::<Vec<ToDoList>>();
        Ok(result)
    }

    // get todo list by id
    pub async fn find(client: &Client, id: i32) -> Result<ToDoList> {
        let stmt = client
            .prepare("select * from todo_list where id = $1 order by id desc")
            .await
            .map_err(AppError::from)?;
        let result = client
            .query(&stmt, &[&id])
            .await
            .map_err(AppError::from)?
            .iter()
            .map(|row| ToDoList::from_row_ref(row).unwrap())
            .collect::<Vec<ToDoList>>()
            .pop()
            .ok_or(AppError::not_found())?;
        Ok(result)
    }

    // update todo list by id
    pub async fn update(client: &Client, list: UpdateTodoList) -> Result<bool> {
        let stmt = client
            .prepare("update todo_list set title = $2 where id = $1")
            .await
            .map_err(AppError::from)?;
        let result = client
            .execute(&stmt, &[&list.title, &list.id])
            .await
            .map_err(AppError::from)?;
        Ok(result > 0)
    }

    // delete todo list by id(cascade delete todo item)
    pub async fn delete(client: &mut Client, id: i32) -> Result<bool> {
        // init transaction
        let tx = client.transaction().await.map_err(AppError::from)?;
        // delete todo list by id
        let stmt = tx
            .prepare("delete from todo_list where id = $1")
            .await
            .map_err(AppError::from)?;
        let result = tx.execute(&stmt, &[&id]).await;
        // rollback when some error happened
        if let Err(err) = result {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::db_error(err));
        }
        // cascade delete todo item
        let stmt = tx
            .prepare("delete from todo_item where id = $1")
            .await
            .map_err(AppError::from)?;
        let result = tx.execute(&stmt, &[&id]).await;
        // rollback when some error happened
        if let Err(err) = result {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::db_error(err));
        }
        tx.commit().await.map_err(AppError::from)?;
        Ok(true)
    }
}
