use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rocket::Route;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: Option<String>,
    pub path: String,
    pub get_available: bool,
    pub post_available: bool,
    pub put_available: bool,
    pub delete_available: bool,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
}