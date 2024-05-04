use serde_json::json;
use warp::Reply;

use crate::auth::models::{AuthUser, Role};
use crate::environment::Environment;
use crate::error::{AuthError, UserError};
use crate::users::models::{PasswordUpdateRequest, UserCreateRequest, UserUpdateRequest};
use crate::users::service;
use crate::WebResult;

