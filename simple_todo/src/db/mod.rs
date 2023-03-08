pub mod todo_list;
pub mod todo_item;

pub use todo_list::*;
pub use todo_item::*;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{types::ToSql, GenericClient, Statement};

use crate::{error::AppError, Result};

pub async fn get_stmt<C: GenericClient>(client: &C, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

pub async fn query<C, T>(client: &C, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<T>>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

pub async fn query_one<C, T>(client: &C, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<T>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let result = query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::not_found())?;
    Ok(result)
}

pub async fn execute<C>(client: &C, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64>
where
    C: GenericClient,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .execute(&stmt, params)
        .await
        .map_err(AppError::from)?;
    Ok(result)
}
