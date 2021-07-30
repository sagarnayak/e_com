use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpiredBlockedToken {
    pub id: String,
    pub token: String,
    pub reason: Option<String>,
    pub created: DateTime<Utc>,
}