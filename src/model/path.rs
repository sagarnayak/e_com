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

impl Path {
    pub fn new(path: &str, readable_path: &str) -> Self {
        Path {
            id: None,
            path: path.to_owned(),
            readable_path: readable_path.to_owned(),
            get_available: false,
            post_available: false,
            put_available: false,
            delete_available: false,
            can_delegate_get: false,
            can_delegate_post: false,
            can_delegate_put: false,
            can_delegate_delete: false,
            force_delegate_get: false,
            force_delegate_post: false,
            force_delegate_put: false,
            force_delegate_delete: false,
            can_access_for_children_get: false,
            can_access_for_children_post: false,
            can_access_for_children_put: false,
            can_access_for_children_delete: false,
            created: None,
            modified: None,
        }
    }

    pub fn get_available(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: true,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn post_available(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: true,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn put_available(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: true,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn delete_available(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: true,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_delegate_get(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: true,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_delegate_post(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: true,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_delegate_put(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: true,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_delegate_delete(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: true,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn force_delegate_get(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: true,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn force_delegate_post(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: true,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn force_delegate_put(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: true,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn force_delegate_delete(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: true,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_access_for_children_get(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: true,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_access_for_children_post(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: true,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_access_for_children_put(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: true,
            can_access_for_children_delete: self.can_access_for_children_delete,
            created: None,
            modified: None,
        }
    }

    pub fn can_access_for_children_delete(&self) -> Self {
        Path {
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            readable_path: self.readable_path.to_owned(),
            get_available: self.get_available,
            post_available: self.post_available,
            put_available: self.put_available,
            delete_available: self.delete_available,
            can_delegate_get: self.can_delegate_get,
            can_delegate_post: self.can_delegate_post,
            can_delegate_put: self.can_delegate_put,
            can_delegate_delete: self.can_delegate_delete,
            force_delegate_get: self.force_delegate_get,
            force_delegate_post: self.force_delegate_post,
            force_delegate_put: self.force_delegate_put,
            force_delegate_delete: self.force_delegate_delete,
            can_access_for_children_get: self.can_access_for_children_get,
            can_access_for_children_post: self.can_access_for_children_post,
            can_access_for_children_put: self.can_access_for_children_put,
            can_access_for_children_delete: true,
            created: None,
            modified: None,
        }
    }
}