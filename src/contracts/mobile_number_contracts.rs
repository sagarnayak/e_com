use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::mobile_number::MobileNumber;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait MobileNumberContracts {
    async fn find_mobile_number_with_id(mobile_number_id: &str, db_pool: &State<DbPool>)
                                        -> Result<MobileNumber, StatusMessage>;
}