use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CachedAuthData {
    pub id: String,
    pub auth_string: String,
    pub exp: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}