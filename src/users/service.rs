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

pub async fn get_users(connection: &PgPool) -> Result<Option<Vec<User>>, Rejection> {
    let result = query_as_unchecked!(
        User,
        r#"SELECT id, email, name, password, role, created_at, updated_at FROM users"#
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn create_user(_req: UserCreateRequest, connection: &PgPool) -> Result<u64, DatabaseError> {
    let result = query_unchecked!(
        r#"INSERT INTO users (id, email, name, password, role, created_at) VALUES ($1, $2, $3, $4, $5, $6)"#,
        uuid::Uuid::new_v4(),
        _req.email,
        _req.name,
        _req.password,
        _req.role.unwrap().to_string(),
        Utc::now()
    )
        .execute(connection)
        .await
        .map(|_| 0)
        .map_err(|_e| {
            let _reply = match _e.as_database_error() {
                None => println!("ERR"),
                Some(err) => {
                    println!("ERR {:?}", err.message().to_string());
                    return DatabaseError { message: err.message().to_string() };
                }
            };
            return DatabaseError { message: String::from("test") };
        });

    return result;
}