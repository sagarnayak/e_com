use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthRolesCrossPathsRequest {
    pub parent_id: String,
    pub get_allowed: bool,
    pub post_allowed: bool,
    pub put_allowed: bool,
    pub delete_allowed: bool,
    pub can_delegate_get: bool,
    pub can_delegate_post: bool,
    pub can_delegate_put: bool,
    pub can_delegate_delete: bool,
    pub where_replacement: Option<String>,
}