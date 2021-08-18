use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshClaims {
    pub owner: String,
    pub jwt_hash: String,
    pub exp: usize,
}