use std::io::Cursor;

use async_trait::async_trait;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::model::status_message::StatusMessage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MigrationStruct;

#[async_trait]
impl MigrationContracts for MigrationStruct {
    async fn may_create_users_table(
        db_pool: DbPool
    ) -> Result<Json<String>, Json<StatusMessage>> {
        let client = match db_pool.pool.get().await {
            Ok(client_positive) => client_positive,
            Err(error) => return Err(
                Json(StatusMessage::bad_req(error.to_string()))
            ),
        };

        let statement = match client
            .prepare_cached(&format!(
                "CREATE TABLE IF NOT EXISTS users(\
                id uuid,\
                name varchar(100) NOT NULL,\
                email varchar(100) NOT NULL,\
                PRIMARY KEY (id) )"
            )
            )
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return Err(
                Json(StatusMessage::bad_req(error.to_string()))
            ),
        };

        let results = match client.execute(
            &statement,
            &[],
        ).await {
            Ok(positive) => positive,
            Err(error) => return Err(
                Json(StatusMessage::bad_req(error.to_string()))
            )
        };

        println!("the result is :: {}", results);

        Ok(Json("Done".to_string()))
    }
}