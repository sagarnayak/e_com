use chrono::{DateTime, Utc};
use postgres::Row;
use rocket::http::Status;
use uuid::Uuid;

use crate::contracts::blocked_for_platform_authorization_contracts::BlockedForPlatformAuthorizationContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::status_message::StatusMessage;

impl BlockedForPlatformAuthorization {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<BlockedForPlatformAuthorization>, StatusMessage> {
        let mut results_to_send: Vec<BlockedForPlatformAuthorization> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get id ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let jwt_hash: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get jwt_hash ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let done: bool = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get enabled ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let created: DateTime<Utc> = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get created ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        None
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };

            let res = BlockedForPlatformAuthorization {
                id: id.to_hyphenated().to_string(),
                jwt_hash,
                done,
                created,
                modified,
            };

            results_to_send.push(res);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl BlockedForPlatformAuthorizationContracts for BlockedForPlatformAuthorization {
    async fn add_jwt(jwt: &String, db_pool: &DbPool) -> Result<bool, StatusMessage> {
        let jwt = jwt.split(".").collect::<Vec<&str>>()[2];

        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!(
            "INSERT INTO blocked_for_platform_authorization \
            (jwt_hash) \
            VALUES ('{}')",
            &jwt
        );

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let _ = match client.execute(&statement, &[]).await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        Ok(
            true
        )
    }

    async fn find_data(jwt: &String, db_pool: &DbPool) -> Result<BlockedForPlatformAuthorization, StatusMessage> {
        let jwt = jwt.split(".").collect::<Vec<&str>>()[2];

        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!(
            "SELECT * FROM blocked_for_platform_authorization \
            WHERE done = false AND jwt_hash = '{}'",
            &jwt
        );

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results_to_send = match BlockedForPlatformAuthorization::convert_results_to_models(&results).await {
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
                    code: Status::NoContent.code,
                    status: Status::NoContent,
                    message: "No data".to_owned(),
                }
            )
        }
    }
}