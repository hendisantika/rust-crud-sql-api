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