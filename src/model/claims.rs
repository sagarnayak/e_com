use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub owner: String,
    pub authorizations_minified: Vec<String>,
    pub exp: usize,
}