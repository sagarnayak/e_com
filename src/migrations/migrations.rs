use async_trait::async_trait;
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
    async fn may_create_table_rows_count_table(db_pool: &DbPool) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS table_rows_counts(\
                    id uuid default gen_random_uuid(),\
                    table_name varchar(100) NOT NULL,\
                    rows integer NOT NULL,\
                    where_condition varchar(100),\
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

    async fn may_create_paths_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS paths(\
                    id uuid default gen_random_uuid(),\
                    path varchar(100) NOT NULL,\
                    get_available bool NOT NULL default false,\
                    post_available bool NOT NULL default false,\
                    put_available bool NOT NULL default false,\
                    delete_available bool NOT NULL default false,\
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
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS roles(\
                    id uuid default gen_random_uuid(),\
                    derived_from uuid,\
                    name varchar(100) NOT NULL UNIQUE,\
                    can_delegate bool NOT NULL default false,\
                    enabled bool NOT NULL default true,\
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

    async fn may_create_auth_roles_cross_paths_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS auth_roles_cross_paths(\
                    id uuid default gen_random_uuid(),\
                    auth_role uuid NOT NULL,\
                    path_id uuid NOT NULL,\
                    path varchar(200) NOT NULL,\
                    get_allowed bool NOT NULL default false,\
                    post_allowed bool NOT NULL default false,\
                    put_allowed bool NOT NULL default false,\
                    delete_allowed bool NOT NULL default false,\
                    can_delegate_get bool NOT NULL default false,\
                    can_delegate_post bool NOT NULL default false,\
                    can_delegate_put bool NOT NULL default false,\
                    can_delegate_delete bool NOT NULL default false,\
                    where_replacement varchar(100),\
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

    async fn may_create_mobile_numbers_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS mobile_numbers(\
                    id uuid default gen_random_uuid(),\
                    country_code varchar(200) NOT NULL,\
                    number varchar(100) NOT NULL,\
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

    async fn may_create_users_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS users(\
                    id uuid default gen_random_uuid(),\
                    role uuid NOT NULL,\
                    password varchar(200) NOT NULL,\
                    first_name varchar(100) NOT NULL,\
                    last_name varchar(100),\
                    email_id varchar(100) NOT NULL UNIQUE,\
                    mobile_number uuid,\
                    enabled bool NOT NULL default true,\
                    created timestamptz default CURRENT_TIMESTAMP NOT NULL,\
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

    async fn may_create_expired_blocked_tokens_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS expired_blocked_tokens(\
                    id uuid default gen_random_uuid(),\
                    token varchar(500) NOT NULL,\
                    reason varchar(200),\
                    created timestamptz default CURRENT_TIMESTAMP NOT NULL,\
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

    async fn may_create_authorization_exceptions_table(
        db_pool: &DbPool
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS authorization_exceptions(\
                    id uuid default gen_random_uuid(),\
                    blocked_user_id varchar(100) NOT NULL,\
                    valid_from timestamptz,\
                    valid_to timestamptz,\
                    reason varchar(200),\
                    created timestamptz default CURRENT_TIMESTAMP NOT NULL,\
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

    async fn may_create_blocked_for_platform_authorization_table(db_pool: &DbPool) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(
                &format!(
                    "CREATE TABLE IF NOT EXISTS blocked_for_platform_authorization(\
                    id uuid default gen_random_uuid(),\
                    user_id uuid NOT NULL,\
                    jwt_hash varchar(200) NOT NULL,\
                    done bool NOT NULL default false,\
                    created timestamptz default CURRENT_TIMESTAMP NOT NULL,\
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