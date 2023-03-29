pub mod db_error;

pub use db_error::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Some error occurred while executing db logic")]
    DbError(#[from] db_error::DbError),
}
