use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub in_home: Option<bool>,
}