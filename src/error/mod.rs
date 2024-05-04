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