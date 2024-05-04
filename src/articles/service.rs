use chrono::Utc;
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use uuid;
use warp::Rejection;

use crate::articles::models::{Article, Comment};

pub async fn get_article_by_id(_id: uuid::Uuid, connection: &PgPool) -> Result<Option<Article>, Rejection> {
    println!("[get_article_by_id] _id: {}", _id);
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, content, created_at, updated_at, in_home FROM articles WHERE id=$1"#,
        &_id
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}