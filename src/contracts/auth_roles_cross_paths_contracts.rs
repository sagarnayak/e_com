use async_trait::async_trait;

use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait AuthRolesCrossPathsContracts {
    async fn find_auth_roles_cross_paths_for_role_id(role_id: &str, db_pool: &DbPool)
                                                     -> Result<Vec<AuthRolesCrossPaths>, StatusMessage>;
}