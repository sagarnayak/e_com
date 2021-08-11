use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockedForPlatformAuthorization {
    pub id: String,
    pub jwt_hash: String,
    pub done: bool,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}