use async_trait::async_trait;

use crate::database::db_pool::DbPool;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait BlockedForPlatformAuthorizationContracts {
    async fn add_jwt(user_id: &String, jwt: &String, nonce: &String, db_pool: &DbPool)
                     -> Result<bool, StatusMessage>;
    async fn find_data_with_jwt(jwt: &String, db_pool: &DbPool)
                                -> Result<BlockedForPlatformAuthorization, StatusMessage>;
    async fn find_data_with_user_id(user_id: &String, db_pool: &DbPool)
                                    -> Result<BlockedForPlatformAuthorization, StatusMessage>;
    async fn done_authorization_for_jwt_hash(jwt_hash: &str, db_pool: &DbPool)
                                             -> Result<bool, StatusMessage>;
}