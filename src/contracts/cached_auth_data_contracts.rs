use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::cached_auth_data::CachedAuthData;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait CachedAuthDataContracts {
    async fn insert_new(auth_string: &str, exp: DateTime<Utc>, db_pool: &DbPool)
                        -> Result<String, StatusMessage>;
    async fn get_data(auth_data_id: &String, db_pool: &DbPool)
                      -> Result<CachedAuthData, StatusMessage>;
}