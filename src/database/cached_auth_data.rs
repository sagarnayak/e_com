use chrono::{DateTime, Utc};
use postgres::Row;
use rocket::http::Status;
use uuid::Uuid;

use crate::contracts::auth_roles_cross_paths_contracts::AuthRolesCrossPathsContracts;
use crate::contracts::cached_auth_data_contracts::CachedAuthDataContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::cached_auth_data::CachedAuthData;
use crate::model::status_message::StatusMessage;

impl CachedAuthData {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<CachedAuthData>, StatusMessage> {
        let mut results_to_send: Vec<CachedAuthData> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let auth_string: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get auth_string ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let exp: DateTime<Utc> = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get exp ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get created ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let res = CachedAuthData {
                id: id.to_hyphenated().to_string(),
                auth_string,
                exp,
                created,
                modified,
            };

            results_to_send.push(res);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl CachedAuthDataContracts for CachedAuthData {
    async fn insert_new(auth_string: &str, exp: DateTime<Utc>, db_pool: &DbPool) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;
        let my_uuid = Uuid::new_v4();
        let statement_to_send =
            &format!(
                "INSERT INTO cached_auth_data (id,auth_string,exp) VALUES ('{}','{}','{}')",
                &my_uuid,
                &auth_string,
                &exp
            );
        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(error.to_string());
            }
        };
        let results = match client.execute(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        if results != 0 {
            Ok(my_uuid.to_hyphenated().to_string())
        } else {
            StatusMessage::bad_request_400_in_result("Failed to insert data".to_owned())
        }
    }

    async fn get_data(auth_data_id: &String, db_pool: &DbPool) -> Result<CachedAuthData, StatusMessage> {
        let client = resolve_client(db_pool).await;
        let my_uuid = Uuid::new_v4();
        let statement_to_send =
            &format!(
                "SELECT * FROM cached_auth_data WHERE id = '{}'",
                &auth_data_id
            );
        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(error.to_string());
            }
        };
        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results_to_send = match CachedAuthData::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if results_to_send.len() != 0 {
            let res_to_return = results_to_send[0].clone();
            Ok(
                res_to_return
            )
        } else {
            Err(
                StatusMessage {
                    code: Status::BadRequest.code,
                    status: Status::BadRequest,
                    message: "No data".to_owned(),
                    sys_message: None,
                }
            )
        }
    }
}