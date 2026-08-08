use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod handlers;
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("could not hash password")]
    ArgonError,
}

impl warp::reject::Reject for AuthError {}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("jwt token not valid")]
    InvalidJWTToken,
    #[error("jwt token creation failed")]
    JWTTokenCreationFailed,
    #[error("no auth header")]
    NoAuthHeader,
    #[error("invalid auth header")]
    InvalidAuthHeader,
    #[error("no permission")]
    NoPermission,
}

impl warp::reject::Reject for AppError {}

impl From<sqlx::error::Error> for AppError {
    fn from(_err: sqlx::error::Error) -> Self {
        AppError::WrongCredentials
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("could not create user")]
    CreateError,
    #[error("could not update user")]
    UpdateError,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DatabaseError {
    pub message: String,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database error")
    }
}

impl warp::reject::Reject for UserError {}
