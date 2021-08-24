use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenLog {
    pub id: String,
    pub token_hash: String,
    pub use_reason: String,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}

pub enum RefreshTokenUsedReason {
    NormalUse(String),
    TryingToForge(String),
}