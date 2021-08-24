use chrono::{DateTime, Utc};
use postgres::Row;
use rocket::State;
use uuid::Uuid;

use crate::contracts::mobile_number_contracts::MobileNumberContracts;
use crate::contracts::refresh_token_contracts::RefreshTokenContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::mobile_number::MobileNumber;
use crate::model::refresh_token_log::{RefreshTokenLog, RefreshTokenUsedReason};
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

impl RefreshTokenLog {
    async fn convert_results_to_models(results: &Vec<Row>, db_pool: &State<DbPool>) -> Result<Vec<RefreshTokenLog>, StatusMessage> {
        let mut results_to_send: Vec<RefreshTokenLog> = vec![];

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
            let token_hash: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get token_hash ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let use_reason: String = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get use_reason ".to_string());
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

            let res = RefreshTokenLog {
                id: id.to_hyphenated().to_string(),
                token_hash,
                use_reason,
                created,
                modified,
            };

            results_to_send.push(res);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl RefreshTokenContracts for RefreshTokenLog {
    async fn refresh_token_used(
        hash: &String,
        use_reason: RefreshTokenUsedReason,
        db_pool: &State<DbPool>,
    ) -> Result<String, StatusMessage> {
        let client = resolve_client(db_pool).await;
        let use_reason = match use_reason {
            RefreshTokenUsedReason::NormalUse(reason) => { reason }
            RefreshTokenUsedReason::TryingToForge(reason) => { reason }
        };
        let statement_to_send = &format!(
            "INSERT INTO refresh_token_log (token_hash,use_reason) VALUES ('{}','{}')",
            &hash,
            &use_reason,
        );

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.execute(&statement, &[]).await {
            Ok(positive) => positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        if results != 0 {
            Ok("done".to_owned())
        } else {
            StatusMessage::bad_request_400_in_result("Failed to insert the data".to_owned())
        }
    }

    async fn check_if_used(hash: &String, db_pool: &State<DbPool>) -> Result<bool, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM refresh_token_log WHERE token_hash = '{}'", hash);

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

        let results_to_send = match RefreshTokenLog::convert_results_to_models(&results, db_pool).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if results_to_send.len() != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}