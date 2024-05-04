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

pub async fn get_article_by_url(_url: String, connection: &PgPool) -> Result<Option<Article>, Rejection> {
    println!("[get_article_by_url] _url: {}", _url);
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, content, created_at, updated_at, in_home FROM articles WHERE url=$1"#,
        _url
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn get_home_article_headers(connection: &PgPool) -> Result<Option<Vec<Article>>, Rejection> {
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, '' as content, created_at, updated_at, in_home FROM articles WHERE in_home=true"#
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn get_article_headers(connection: &PgPool) -> Result<Option<Vec<Article>>, Rejection> {
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, '' as content, created_at, updated_at, in_home FROM articles"#
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn create_article(_article: &Article, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    let _result = query_unchecked!(
        r#"INSERT INTO articles (id, title, url, content, created_at, in_home) VALUES ($1, $2, $3, $4, $5, $6)"#,
        _article.id,
        _article.title,
        _article.url,
        _article.content,
        Utc::now(),
        _article.in_home
    )
        .execute(connection)
        .await
        .unwrap();

    Ok(Some(0))
}