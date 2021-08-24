use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
pub trait UserContracts {
    async fn find_user_with_email(email: &String, db_pool: &State<DbPool>)
                                  -> Result<User, StatusMessage>;
    async fn find_user_with_id(id: &String, db_pool: &State<DbPool>)
                               -> Result<User, StatusMessage>;
}