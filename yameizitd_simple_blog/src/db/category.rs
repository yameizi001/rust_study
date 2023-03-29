use db_error::DbError;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{
    db::post,
    error::db_error,
    form::{CreateForm, QueryForm, UpdateForm},
    model::{self, Category},
};

pub async fn insert_category_by_name(
    pool: &Pool<Postgres>,
    form: CreateForm,
) -> Result<model::Category, DbError> {
    tracing::debug!("Insert category by name: {:?}", form.name);
    let record = sqlx::query!(
        r#"insert into simple_blog_category(name) values($1) returning id, name, num"#,
        form.name
    )
    .fetch_one(pool)
    .await?;
    let category = model::Category {
        id: record.id,
        name: record.name,
        num: record.num,
    };
    tracing::debug!("Inserted category: {:?}", category);
    Ok(category)
}

pub async fn delete_by_id(pool: &Pool<Postgres>, id: i64) -> Result<bool, DbError> {
    // strong consistency is not required
    tracing::debug!("Delete category by id: {:?}", id);
    let count = post::exist_by_category_id(pool, id).await?;
    if count > 0 {
        tracing::warn!("Category has been in use by {:?} posts", count);
        Err(DbError::associated_in_use(id.to_string()))
    } else {
        let row = sqlx::query(r#"delete from simple_blog_category where id = $1"#)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(row.rows_affected() > 0)
    }
}

pub async fn update(pool: &Pool<Postgres>, form: UpdateForm) -> Result<bool, DbError> {
    tracing::debug!("Update category name({:?}) by id: {:?}", form.name, form.id);
    let row = sqlx::query(r#"update simple_blog_category set name = $1 where id = $2"#)
        .bind(form.name)
        .bind(form.id)
        .execute(pool)
        .await?;
    Ok(row.rows_affected() > 0)
}

pub async fn select_by_option(
    pool: &Pool<Postgres>,
    form: QueryForm,
) -> Result<Vec<Category>, DbError> {
    tracing::debug!("Select category by option: {:?}", form);

    let sql = format!(
        "select id, name, num from simple_blog_category {} limit $3 offset $4",
        &form.page()
    );
    let records = sqlx::query(&sql)
        .map(|row: PgRow| Category {
            id: row.get::<i64, _>("id"),
            name: row.get::<String, _>("name"),
            num: row.get::<i64, _>("num"),
        })
        .fetch_all(pool)
        .await?;
    Ok(records)
}
