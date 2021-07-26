use async_trait::async_trait;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait UserContracts {
    async fn fetch_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>>;
}