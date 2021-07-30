use serde::{Deserialize, Serialize};

use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub owner: String,
    pub authorizations: Vec<AuthRolesCrossPaths>,
    pub exp: usize,
}