use warp::Filter;
use warp::http::{HeaderMap, HeaderValue};

use crate::{Result, WebResult};
use crate::auth::{BEARER, JWT_SECRET};
use crate::auth::models::{AuthUser, Claims, Role};
use crate::error::AppError;

