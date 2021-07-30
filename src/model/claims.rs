use serde::{Deserialize, Serialize};

use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub owner: String,
    pub authorizations_minified: Vec<String>,
    pub exp: usize,
}