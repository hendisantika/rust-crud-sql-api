use serde_json::json;
use warp::Reply;

use crate::auth::models::{AuthUser, Role};
use crate::environment::Environment;
use crate::error::{AuthError, UserError};
use crate::users::models::{PasswordUpdateRequest, UserCreateRequest, UserUpdateRequest};
use crate::users::service;
use crate::WebResult;

// Returns all users
pub async fn get_users_handler(_env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[get_users_handler] Action performed by {}", _user);
    let result = service::get_users(_env.db()).await?;
    Ok(warp::reply::json(&result))
}

// Returns user with given id
pub async fn get_user_by_id_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    let uuid = uuid::Uuid::parse_str(&_id).unwrap();
    let _result = service::get_user_by_id(uuid, _env.db()).await?;
    println!("[get_user_by_id_handler] id={}, email={}", _id, &_result.clone().unwrap().email);
    Ok(warp::reply::json(&_result))
}
