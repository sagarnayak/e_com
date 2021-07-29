use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::mobile_number::MobileNumber;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait AuthRolesCrossPathsContracts {
    async fn find_auth_roles_cross_paths_for_role_id(role_id: &str, db_pool: &DbPool) -> Result<Vec<AuthRolesCrossPaths>, StatusMessage>;
}