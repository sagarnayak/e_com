use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: String,
    pub derived_from: Option<String>,
    pub name: String,
    pub can_delegate: bool,
    pub can_access_for_children: bool,
    pub enabled: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}