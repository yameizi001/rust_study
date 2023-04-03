use chrono::Utc;
use db_error::DbError;
use sqlx::{Pool, Postgres, Row};

use crate::{
    error::db_error,
    form::{post::UpdateForm, DraftForm, ReleaseForm},
    model::StatusSign,
};

pub async fn insert_draft(pool: &Pool<Postgres>, form: DraftForm) -> Result<i64, DbError> {
    tracing::debug!("Insert a post draft:\n{:#?}", form);
    let record = sqlx::query(
        r#"
        insert into simple_blog_post 
            ( category_id, title, digest, sketch, markdown, html, tags, secret, create_at )
        values
	        ( $1, $2, $3, $4, $5, $6, $7, $8, $9::timestamptz ) 
        returning id;
        "#,
    )
    .bind(form.category_id)
    .bind(form.title)
    .bind(form.digest)
    .bind(form.sketch)
    .bind(form.markdown)
    .bind(form.html)
    .bind(form.tags)
    .bind(form.secret)
    .bind(Utc::now().to_string())
    .fetch_one(pool)
    .await?;
    let id = record.get::<i64, _>("id");
    tracing::debug!("Insert a post draft: {:?}", id);
    Ok(id)
}

pub async fn insert_release(pool: &Pool<Postgres>, form: ReleaseForm) -> Result<i64, DbError> {
    tracing::debug!("Insert a release post:\n{:#?}", form);
    let record = sqlx::query(
        r#"
        insert into simple_blog_post 
            ( category_id, title, digest, sketch, markdown, html, tags, secret, create_at, is_private )
        values
	        ( $1, $2, $3, $4, $5, $6, $7, $8, $9 ) 
        returning id;
        "#,
    )
    .bind(form.category_id)
    .bind(form.title)
    .bind(form.digest)
    .bind(form.sketch)
    .bind(form.markdown)
    .bind(form.html)
    .bind(form.tags)
    .bind(form.secret)
    .bind(form.is_private)
    .fetch_one(pool)
    .await?;
    let id = record.get::<i64, _>("id");
    tracing::debug!("Insert a post draft: {:?}", id);
    Ok(id)
}

pub async fn discard(pool: &Pool<Postgres>, id: i64) -> Result<bool, DbError> {
    tracing::debug!("Discard a post by id: {:?}", id);
    let row = sqlx::query(r#"update simple_blog_post set status_sign = $ 1 where id = $2"#)
        .bind(StatusSign::DISCARD.toi8())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(row.rows_affected() > 0)
}

pub async fn delete(pool: &Pool<Postgres>, id: i64) -> Result<bool, DbError> {
    tracing::debug!("Delete a post by id: {:?}", id);
    let row = sqlx::query(r#"delete simple_blog_post where id = $1"#)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(row.rows_affected() > 0)
}

pub async fn recovery(pool: &Pool<Postgres>, id: i64) -> Result<bool, DbError> {
    tracing::debug!("Recovery a post by id: {:?}", id);
    let row = sqlx::query(
        r#"update simple_blog_post set status_sign = $ 1 where id = $2 and status_sign = '-1'"#,
    )
    .bind(StatusSign::DRAFT.toi8())
    .bind(id)
    .execute(pool)
    .await?;
    Ok(row.rows_affected() > 0)
}

pub async fn update(pool: &Pool<Postgres>, form: UpdateForm) -> Result<bool, DbError> {
    // strong consistency is not required
    tracing::debug!("Update a post(id:{:?}):\n{:?}", form.id, form);
    let row = sqlx::query(
        r#"
        update 
            simple_blog_post 
        set 
            category_id = $ 1 , title = $2, digest = $3, sketch = $4, markdown = $5, html = $6, tags = $7, secret = $8, status_sign = $9, is_private = $10
        where 
            id = $11
        "#,
    )
    .bind(StatusSign::DRAFT.toi8())
    .execute(pool)
    .await?;
    Ok(row.rows_affected() > 0)
}

pub async fn exist_by_category_id(pool: &Pool<Postgres>, category_id: i64) -> Result<i64, DbError> {
    tracing::debug!("Exist post by category_id: {:?}?", category_id);
    let row =
        sqlx::query(r#"select count(*) "count" from simple_blog_post where category_id = $1"#)
            .bind(category_id)
            .fetch_one(pool)
            .await?;
    let count = row.get::<i64, _>("count");
    tracing::debug!("Category id used by {:?} posts", count);
    Ok(count)
}
