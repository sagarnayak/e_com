use serde::{Deserialize, Serialize};

use crate::model::role::Role;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub owner: String,
    pub role: Role,
    pub exp: usize,
}