use chrono::Utc;
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use uuid;
use warp::Rejection;

use crate::error::{AuthError, DatabaseError};
use crate::users::models::{User, UserCreateRequest, UserUpdateRequest};

pub async fn get_user_by_id(_id: uuid::Uuid, connection: &PgPool) -> Result<Option<User>, Rejection> {
    let user = query_as_unchecked!(
        User,
        r#"SELECT id, email, name, password, role, created_at, updated_at FROM users WHERE id = $1"#,
        _id
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| {
            AuthError::InvalidCredentials
        })
        .ok();
    Ok(user)
}

pub async fn get_user_by_email(email: &str, connection: &PgPool) -> Result<Option<User>, Rejection> {
    let user = query_as_unchecked!(
        User,
        r#"SELECT id, email, name, password, role, created_at, updated_at FROM users WHERE email = $1"#,
        email
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| {
            AuthError::InvalidCredentials
        })
        .ok();
    Ok(user)
}