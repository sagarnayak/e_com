use async_trait::async_trait;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait MigrationContracts {
    async fn may_create_users_table(db_pool: DbPool) -> Result<Json<String>, Json<StatusMessage>>;
}