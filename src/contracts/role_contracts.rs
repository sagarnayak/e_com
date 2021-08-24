use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::page_response::PageResponse;
use crate::model::role::Role;
use crate::model::role_request::RoleRequest;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;
use uuid::Uuid;

#[async_trait]
pub trait RoleContracts {
    async fn find_role_for(user: &User, db_pool: &State<DbPool>) -> Result<Role, StatusMessage>;
    async fn find_role_for_role_id(role_id: &str, db_pool: &State<DbPool>)
                                   -> Result<Role, StatusMessage>;
    async fn find_role_for_admin(db_pool: &DbPool) -> Result<Role, StatusMessage>;
    async fn add_role(user_role: &Role, role: &RoleRequest, db_pool: &DbPool)
                      -> Result<Uuid, StatusMessage>;
    async fn find_roles_created_by_role(
        role: &Role,
        page_number: &u32,
        page_size: &u32,
        db_pool: &State<DbPool>,
    ) -> Result<PageResponse<Role>, StatusMessage>;
    async fn if_role_created_by(
        role_id_to_check: &String,
        parent_role_id: &String,
        db_pool: &State<DbPool>,
    )
        -> Result<bool, StatusMessage>;
}