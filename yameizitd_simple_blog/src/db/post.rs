use db_error::DbError;
use sqlx::{Pool, Postgres, Row};

use crate::error::db_error;

pub async fn exist_by_category_id(pool: &Pool<Postgres>, category_id: i64) -> Result<i64, DbError> {
    tracing::debug!("Exist post by category_id: {}?", category_id);
    let row =
        sqlx::query(r#"select count(*) "count" from simple_blog_post where category_id = $1"#)
            .bind(category_id)
            .fetch_one(pool)
            .await?;
    let count = row.get::<i64, _>("count");
    tracing::debug!("Category id used by {} posts", count);
    Ok(count)
}
