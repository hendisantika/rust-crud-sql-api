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

// Creates new user. Same logic as in registration service.
pub async fn user_create_handler(mut _req: UserCreateRequest, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    match service::get_user_by_email(&_req.email, _env.db()).await {
        Ok(None) => (),
        Ok(existing) => {
            println!("[user_create_handler] User {} already exists", &existing.unwrap().email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to create user, email already registered"})));
        }
        _ => (),
    }

    let hash = _env.argon().hasher().with_password(&_req.password).hash().unwrap();
    _req.password = hash;
    _req.role = Some(Role::User);

    let email = _req.email.clone();
    match service::create_user(_req, _env.db()).await {
        Err(e) => {
            println!("[user_create_handler] Error creating user {}: {:?}", &email, e.message);
            return Err(warp::reject::custom(UserError::CreateError));
        }
        _ => {
            println!("[user_create_handler] User creation successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}

// Updates user
pub async fn user_update_handler(_req: UserUpdateRequest, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[user_update_handler][{}] Updating user {}", _user, &_req.email);
    service::update_user(_req, _env.db()).await.map(|_e| UserError::UpdateError);
    Ok(warp::reply::json(&json!({"status":"success", "message":"User updated"})))
}
