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