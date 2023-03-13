use tokio_postgres::Client;

use crate::{
    model::{CreateUrl, Url, UrlID, UrlTarget},
    Result,
};

use super::{query, query_one};

pub async fn create(client: &Client, url: CreateUrl, id: String) -> Result<UrlID> {
    let result = query_one(client, "SELECT id FROM url WHERE id = $1", &[&id]).await;
    match result {
        Ok(result) => return Ok(result),
        Err(err) if !err.is_not_found() => return Err(err),
        _ => {}
    };
    let result = query_one(
        client,
        "INSERT INTO url(id, url, email) VALUES ($1, $2, $3) RETURNING id",
        &[&id, &url.url, &url.email],
    )
    .await?;
    Ok(result)
}

pub async fn goto_url(client: &mut Client, id: String) -> Result<UrlTarget> {
    let result = query_one(
        client,
        "UPDATE url SET visit = visit + 1 WHERE id = $1 RETURNING url",
        &[&id],
    )
    .await?;
    Ok(result)
}

pub async fn rank(client: &Client) -> Result<Vec<Url>> {
    let result = query(
        client,
        "select * from url where is_del = false order by visit desc",
        &[],
    )
    .await?;
    Ok(result)
}
