use chrono::{DateTime, Utc};
use rocket::State;

use crate::contracts::user_contracts::UserContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[async_trait]
impl UserContracts for Role {
    async fn find_user_with_email(_: String, db_pool: &State<DbPool>) -> Result<User, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement = match client
            .prepare_cached(&format!("SELECT * FROM users WHERE email"))
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let mut roles: Vec<Role> = vec![];

        for row in results {
            let id: String = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let derived_from: String = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get derived_from ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let name: String = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get name ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_delegate: bool = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_delegate ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let path: String = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get path ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let read: bool = match row.try_get(5) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get read ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let write: bool = match row.try_get(6) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get write ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let edit: bool = match row.try_get(7) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get edit ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let delete: bool = match row.try_get(8) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get delete ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let identifier_required: bool = match row.try_get(9) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get identifier_required ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let identifier: String = match row.try_get(10) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get identifier ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let where_replacement: String = match row.try_get(11) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get where_replacement ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let enabled: bool = match row.try_get(12) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get enabled ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let valid_from: DateTime<Utc> = match row.try_get(13) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get valid_from ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let valid_to: DateTime<Utc> = match row.try_get(14) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get valid_to ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(15) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get created ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: DateTime<Utc> = match row.try_get(16) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get modified ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let role = Role {
                id,
                derived_from,
                name,
                can_delegate,
                path,
                read,
                write,
                edit,
                delete,
                identifier_required,
                identifier,
                where_replacement,
                enabled,
                valid_from,
                valid_to,
                created,
                modified,
            };

            roles.push(role);
        }

        if roles.len() != 0 {
            // let _: Role = roles[0].clone();
            /*Ok(
                role_to_return
            )*/
            panic!();
        } else {
            StatusMessage::bad_request_400_in_result(
                "Failed to get role".to_owned()
            )
        }
    }
}