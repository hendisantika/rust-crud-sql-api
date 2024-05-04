use crate::auth::models::{Claims, Role};
use crate::Result;

pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";