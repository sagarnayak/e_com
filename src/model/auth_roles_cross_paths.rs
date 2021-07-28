use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthRolesCrossPaths {
    pub id: String,
    pub auth_role: String,
    pub path: String,
    pub get_allowed: bool,
    pub post_allowed: bool,
    pub put_allowed: bool,
    pub delete_allowed: bool,
    pub where_replacement: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}