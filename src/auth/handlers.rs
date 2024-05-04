use serde_json::json;
use warp::Reply;

use crate::{users, WebResult};
use crate::auth::create_jwt;
use crate::auth::models::{LoginRequest, LoginResponse, Role};
use crate::environment::Environment;
use crate::error::AuthError;
use crate::users::models::UserCreateRequest;

pub async fn register_handler(mut _req: UserCreateRequest, _env: Environment) -> WebResult<impl Reply> {
    match users::service::get_user_by_email(&_req.email, _env.db()).await {
        Ok(None) => (),
        Ok(existing) => {
            println!("[register_handler] User {} already exists", &existing.unwrap().email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to complete registration, email already registered"})))
        },
        _ => (),
    }

    let hash = _env.argon().hasher().with_password(&_req.password).hash().unwrap();
    _req.password = hash;
    _req.role = Some(Role::User);

    let email = _req.email.clone();
    match users::service::create_user(_req, _env.db()).await {
        Err(e) => {
            println!("[register_handler] Error registering user {}: {:?}", &email, e.message);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Registration error"})))
        },
        _ => {
            println!("[register_handler] Registration successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}