use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Associated field value({field_val:?}) is in use")]
    AssociatedFieldInUse { field_val: String },
    #[error("Some error occurred while executing sql")]
    SqlxPostgresError(#[from] sqlx::error::Error),
    #[error("Some error occurred while convert PgType into i16: {cause:?}")]
    DbTypeConvertError { cause: String },
}

impl DbError {
    pub fn associated_in_use(field_val: String) -> Self {
        DbError::AssociatedFieldInUse { field_val }
    }
}
