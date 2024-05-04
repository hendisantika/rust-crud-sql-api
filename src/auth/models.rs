use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::users::models::User;

// Parsed user from JWT session that is injected in authenticated handlers
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub id: String,
    pub role: Role,
    pub login_at: DateTime<Utc>,
}
