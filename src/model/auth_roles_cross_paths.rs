use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthRolesCrossPaths {
    pub id: String,
    pub auth_role: Option<String>,
    pub path_id: Option<String>,
    pub path: String,
    pub get_allowed: bool,
    pub post_allowed: bool,
    pub put_allowed: bool,
    pub delete_allowed: bool,
    pub can_delegate_get: bool,
    pub can_delegate_post: bool,
    pub can_delegate_put: bool,
    pub can_delegate_delete: bool,
    pub can_access_for_children_get: bool,
    pub can_access_for_children_post: bool,
    pub can_access_for_children_put: bool,
    pub can_access_for_children_delete: bool,
    pub where_replacement: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
}

impl AuthRolesCrossPaths {
    pub fn get_minified_version(&self) -> String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.id,
            "0".to_owned(),
            "0".to_owned(),
            self.path,
            if self.get_allowed { "1" } else { "0" },
            if self.post_allowed { "1" } else { "0" },
            if self.put_allowed { "1" } else { "0" },
            if self.delete_allowed { "1" } else { "0" },
            if self.can_delegate_get { "1" } else { "0" },
            if self.can_delegate_post { "1" } else { "0" },
            if self.can_delegate_put { "1" } else { "0" },
            if self.can_delegate_delete { "1" } else { "0" },
            if self.can_access_for_children_get { "1" } else { "0" },
            if self.can_access_for_children_post { "1" } else { "0" },
            if self.can_access_for_children_put { "1" } else { "0" },
            if self.can_access_for_children_delete { "1" } else { "0" },
            match &self.where_replacement {
                Some(positive) => {
                    positive.to_owned()
                }
                None => {
                    "0".to_owned()
                }
            },
            "0".to_owned(),
            "0".to_owned(),
        )
    }
    pub fn full_version(minified_version: String) -> Self {
        let splits = minified_version.split("_").collect::<Vec<&str>>();
        Self {
            id: splits[0].to_owned(),
            auth_role: None,
            path_id: None,
            path: splits[3].to_owned(),
            get_allowed: if splits[4].to_owned() == "1" { true } else { false },
            post_allowed: if splits[5].to_owned() == "1" { true } else { false },
            put_allowed: if splits[6].to_owned() == "1" { true } else { false },
            delete_allowed: if splits[7].to_owned() == "1" { true } else { false },
            can_delegate_get: if splits[8].to_owned() == "1" { true } else { false },
            can_delegate_post: if splits[9].to_owned() == "1" { true } else { false },
            can_delegate_put: if splits[10].to_owned() == "1" { true } else { false },
            can_delegate_delete: if splits[11].to_owned() == "1" { true } else { false },
            can_access_for_children_get: if splits[12].to_owned() == "1" { true } else { false },
            can_access_for_children_post: if splits[13].to_owned() == "1" { true } else { false },
            can_access_for_children_put: if splits[14].to_owned() == "1" { true } else { false },
            can_access_for_children_delete: if splits[15].to_owned() == "1" { true } else { false },
            where_replacement: if splits[16].to_owned() != "0" { Some(splits[16].to_owned()) } else { None },
            created: None,
            modified: None,
        }
    }
}