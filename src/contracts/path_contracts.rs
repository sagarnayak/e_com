use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::path::Path;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait PathContracts {
    async fn fetch_all(db_pool: DbPool) -> Result<Vec<Path>, StatusMessage>;
}