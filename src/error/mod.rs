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
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation failed")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}

impl warp::reject::Reject for AppError {}

impl From<sqlx::error::Error> for AppError {
    fn from(_err: sqlx::error::Error) -> Self {
        AppError::WrongCredentialsError
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}