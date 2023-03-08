pub mod todo_item;
pub mod todo_list;
pub mod usage;

use deadpool_postgres::Client;
pub use todo_item::*;
pub use todo_list::*;
pub use usage::*;

use crate::{error::AppError, AppState, Result};

pub async fn get_client(state: &AppState, handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("Handler[{}] get pool client err: {:?}", handler_name, err);
        AppError::db_error(err)
    })
}

pub fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("Handler[{}] error: {:?}", handler_name, err);
        err
    })
}
