use chrono::{DateTime, Utc};
use rocket::Route;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationException {
    pub id: String,
    pub blocked_user_id: String,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub reason: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}