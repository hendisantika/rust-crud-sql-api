use serde_json::json;
use warp::Reply;

use crate::articles::models::{Article, Comment};
use crate::articles::service;
use crate::auth::models::AuthUser;
use crate::environment::Environment;
use crate::WebResult;

pub async fn get_article_by_url_handler(_url: String, _env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_article_by_url(_url, _env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_home_article_headers_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_home_article_headers(_env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_article_headers_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_article_headers(_env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn create_article_handler(mut _req: Article, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    _req.id = uuid::Uuid::new_v4().into();
    if _req.in_home == None {
        _req.in_home = Some(false);
    }
    let _result = service::create_article(&_req, _env.db()).await?;
    println!("[create_article_handler] Created article '{}'", &_req.title.unwrap());
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article saved"})))
}