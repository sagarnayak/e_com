use async_trait::async_trait;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;

#[async_trait]
pub trait TableRowsCountContracts {
    async fn insert_count(
        table_name: &str,
        where_condition: &Option<String>,
        db_pool: &State<DbPool>,
    ) -> Result<u32, StatusMessage>;
    async fn find_count(
        table_name: &str,
        where_condition: &Option<String>,
        db_pool: &State<DbPool>,
    ) -> Result<u32, StatusMessage>;
    async fn clear_data_for_table(
        table_name: &str,
        db_pool: &State<DbPool>,
    ) -> Result<bool, StatusMessage>;
}