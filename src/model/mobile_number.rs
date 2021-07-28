use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MobileNumber {
    pub id: String,
    pub country_code: String,
    pub number: String,
}