use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: Option<String>,
    pub path: String,
    pub readable_path: String,
    pub get_available: bool,
    pub post_available: bool,
    pub put_available: bool,
    pub delete_available: bool,
    pub can_delegate_get: bool,
    pub can_delegate_post: bool,
    pub can_delegate_put: bool,
    pub can_delegate_delete: bool,
    pub force_delegate_get: bool,
    pub force_delegate_post: bool,
    pub force_delegate_put: bool,
    pub force_delegate_delete: bool,
    pub can_access_for_children_get: bool,
    pub can_access_for_children_post: bool,
    pub can_access_for_children_put: bool,
    pub can_access_for_children_delete: bool,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
}