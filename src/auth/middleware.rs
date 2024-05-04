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

// Decodes JWT from header, checks its validity and assembles User object to be passed to the handlers
async fn authorize_any(headers: HeaderMap<HeaderValue>) -> WebResult<AuthUser> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
                &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
            )
                .map_err(|_| warp::reject::custom(AppError::JWTTokenError))?;

            let user = AuthUser::new(decoded.claims.sub, decoded.claims.role);
            Ok(user)
        }
        Err(e) => return Err(warp::reject::custom(AppError::from(e))),
    }
}