use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub role: String,
    pub password:String,
    pub name: String,
    pub email_id: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}