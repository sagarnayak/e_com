use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::refresh_token_log::RefreshTokenUsedReason;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait RefreshTokenContracts {
    async fn refresh_token_used(hash: &String, use_reason: RefreshTokenUsedReason, db_pool: &State<DbPool>)
                                -> Result<String, StatusMessage>;
    async fn check_if_used(hash: &String, db_pool: &State<DbPool>)
                            -> Result<bool, StatusMessage>;
}