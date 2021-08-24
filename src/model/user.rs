use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::mobile_number::MobileNumber;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub role: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email_id: String,
    pub mobile_number: Option<MobileNumber>,
    pub enabled: bool,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}