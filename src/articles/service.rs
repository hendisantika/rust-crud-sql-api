use chrono::Utc;
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use uuid;
use warp::Rejection;

use crate::articles::models::{Article, Comment};

