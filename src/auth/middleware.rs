use warp::Filter;
use warp::http::{HeaderMap, HeaderValue};

use crate::{Result, WebResult};
use crate::auth::{BEARER, JWT_SECRET};
use crate::auth::models::{AuthUser, Claims, Role};
use crate::error::AppError;

// Authentication middleware
pub fn authenticated() -> impl Filter<Extract=(AuthUser, ), Error=warp::reject::Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize_any)
}