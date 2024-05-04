use serde_json::json;
use warp::Reply;

use crate::{users, WebResult};
use crate::auth::create_jwt;
use crate::auth::models::{LoginRequest, LoginResponse, Role};
use crate::environment::Environment;
use crate::error::AuthError;
use crate::users::models::UserCreateRequest;

