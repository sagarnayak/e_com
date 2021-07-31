use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthRolesCrossPaths {
    pub id: String,
    pub auth_role: String,
    pub path_id: String,
    pub path: String,
    pub get_allowed: bool,
    pub post_allowed: bool,
    pub put_allowed: bool,
    pub delete_allowed: bool,
    pub where_replacement: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}

impl AuthRolesCrossPaths {
    pub fn get_minified_version(&self) -> String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.id,
            self.auth_role,
            self.path_id,
            self.path,
            if self.get_allowed { "1" } else { "0" },
            if self.post_allowed { "1" } else { "0" },
            if self.put_allowed { "1" } else { "0" },
            if self.delete_allowed { "1" } else { "0" },
            match &self.where_replacement {
                Some(positive) => {
                    positive.to_owned()
                }
                None => {
                    "0".to_owned()
                }
            },
            self.created,
            match &self.modified {
                Some(positive) => {
                    positive.to_owned().to_string()
                }
                None => {
                    "0".to_owned()
                }
            },
        )
    }
    pub fn full_version(minified_version: String) -> Self {
        let splits = minified_version.split("_").collect::<Vec<&str>>();
        Self {
            id: splits[0].to_owned(),
            auth_role: splits[1].to_owned(),
            path_id: splits[2].to_owned(),
            path: splits[3].to_owned(),
            get_allowed: if splits[4].to_owned() == "1" { true } else { false },
            post_allowed: if splits[5].to_owned() == "1" { true } else { false },
            put_allowed: if splits[6].to_owned() == "1" { true } else { false },
            delete_allowed: if splits[7].to_owned() == "1" { true } else { false },
            where_replacement: if splits[8].to_owned() != "0" { Some(splits[8].to_owned()) } else { None },
            created: splits[9].to_owned().parse::<DateTime<Utc>>().unwrap(),
            modified: if splits[10].to_owned() != "0" { Some(splits[9].to_owned().parse::<DateTime<Utc>>().unwrap()) } else { None },
        }
    }
}