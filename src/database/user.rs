use chrono::{DateTime, Utc};
use rocket::State;
use uuid::Uuid;

use crate::contracts::user_contracts::UserContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
impl UserContracts for User {
    async fn find_user_with_email(email_id: String, db_pool: &State<DbPool>) -> Result<User, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM users WHERE email_id = '{}'", email_id);

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

        let mut results_vec: Vec<User> = vec![];


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
            let role: Uuid = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get role ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let password: String = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get password ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let name: String = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get name ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let email_id: String = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => {
                        return StatusMessage::bad_request_400_in_result("failed to get email_id ".to_string());
                    }
                },
                Err(error) => {
                    let error_message = error.to_string();
                    return StatusMessage::bad_request_400_in_result(error_message);
                }
            };
            let enabled: bool = match row.try_get(5) {
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
            let created: DateTime<Utc> = match row.try_get(6) {
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
            let modified: Option<DateTime<Utc>> = match row.try_get(7) {
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

            /*let user = User {
                id: id.to_hyphenated().to_string(),
                role: role.to_hyphenated().to_string(),
                password,
                name,
                email_id,
                enabled,
                created,
                modified,
            };

            results_vec.push(user);*/
        }

        if results_vec.len() != 0 {
            let user: User = results_vec[0].clone();
            Ok(
                user
            )
        } else {
            StatusMessage::bad_request_400_in_result(
                "User not found.".to_owned()
            )
        }
    }
}