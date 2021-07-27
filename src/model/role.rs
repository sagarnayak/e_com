use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: String,
    pub derived_from: String,
    pub name: String,
    pub can_delegate: bool,
    pub path: String,
    pub read: bool,
    pub write: bool,
    pub edit: bool,
    pub delete: bool,
    pub identifier_required: bool,
    pub identifier: String,
    pub where_replacement: String,
    pub enabled: bool,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}