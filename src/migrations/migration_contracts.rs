use async_trait::async_trait;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait MigrationContracts {
    async fn may_create_cached_auth_data_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_refresh_token_logs_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_table_rows_count_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_paths_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_roles_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_auth_roles_cross_paths_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_mobile_numbers_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_users_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_expired_blocked_tokens_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_authorization_exceptions_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
    async fn may_create_blocked_for_platform_authorization_table(db_pool: &DbPool) -> Result<String, StatusMessage>;
}