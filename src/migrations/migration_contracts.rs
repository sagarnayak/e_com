use async_trait::async_trait;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait MigrationContracts {
    async fn may_create_users_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_roles_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
}