use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::role::Role;
use crate::model::role_request::RoleRequest;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait RoleContracts {
    async fn find_role_for(user: &User, db_pool: &State<DbPool>) -> Result<Role, StatusMessage>;
    async fn find_role_for_admin(db_pool: &DbPool) -> Result<Role, StatusMessage>;
    async fn add_role(user_role: &Role, role: &RoleRequest, db_pool: &DbPool) -> Result<u64, StatusMessage>;
}