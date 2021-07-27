use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::database::db_pool::DbPool;
use crate::migrations::migration_contracts::MigrationContracts;
use crate::model::status_message::StatusMessage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MigrationStruct;

#[async_trait]
impl MigrationContracts for MigrationStruct {
    async fn may_create_users_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = match db_pool.pool.get().await {
            Ok(client_positive) => client_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string()
            ),
        };

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS users(\
                    id uuid default gen_random_uuid(),\
                    role uuid NOT NULL,\
                    name varchar(100) NOT NULL,\
                    email varchar(100) NOT NULL,\
                    created timestamptz default CURRENT_TIMESTAMP,\
                    modified timestamptz,\
                    PRIMARY KEY (id) )"
                )
            )
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string(),
            ),
        };

        let results = match client.execute(
            &statement,
            &[],
        ).await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string(),
            )
        };

        Ok(format!("done :: {}", results))
    }

    async fn may_create_roles_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = match db_pool.pool.get().await {
            Ok(client_positive) => client_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string()
            ),
        };

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS roles(\
                    id uuid default gen_random_uuid(),\
                    derived_from uuid,\
                    name varchar(100) NOT NULL,\
                    can_delegate bool NOT NULL,\
                    path varchar(100) NOT NULL,\
                    read bool NOT NULL,\
                    write bool NOT NULL,\
                    edit bool NOT NULL,\
                    delete bool NOT NULL,\
                    identifier_required bool NOT NULL,\
                    identifier varchar(100),\
                    where_replacement varchar(100),\
                    enabled bool NOT NULL,\
                    valid_from timestamptz,\
                    valid_to timestamptz,\
                    created timestamptz default CURRENT_TIMESTAMP,\
                    modified timestamptz,\
                    PRIMARY KEY (id) )"
                )
            )
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string(),
            ),
        };

        let results = match client.execute(
            &statement,
            &[],
        ).await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(
                error.to_string(),
            )
        };

        Ok(format!("done :: {}", results))
    }
}