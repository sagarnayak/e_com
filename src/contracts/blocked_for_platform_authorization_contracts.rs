use async_trait::async_trait;

use crate::database::db_pool::DbPool;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait BlockedForPlatformAuthorizationContracts {
    async fn add_jwt(jwt: &String, db_pool: &DbPool) -> Result<bool, StatusMessage>;
    async fn find_data(jwt: &String, db_pool: &DbPool) -> Result<BlockedForPlatformAuthorization, StatusMessage>;
}