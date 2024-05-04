use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth::models::Role;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Option<Role>,
}