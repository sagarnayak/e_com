use async_trait::async_trait;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait RoleContracts {
    async fn find_role_for(user: User, db_pool: &State<DbPool>) -> Result<Role, StatusMessage>;
}