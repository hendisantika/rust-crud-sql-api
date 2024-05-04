use chrono::Utc;
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use uuid;
use warp::Rejection;

use crate::error::{AuthError, DatabaseError};
use crate::users::models::{User, UserCreateRequest, UserUpdateRequest};

