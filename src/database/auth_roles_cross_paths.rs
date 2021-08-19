use chrono::{DateTime, Utc};
use postgres::Row;
use uuid::Uuid;

use crate::contracts::auth_roles_cross_paths_contracts::AuthRolesCrossPathsContracts;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::status_message::StatusMessage;

impl AuthRolesCrossPaths {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<AuthRolesCrossPaths>, StatusMessage> {
        let mut results_to_send: Vec<AuthRolesCrossPaths> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let auth_role: Uuid = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get auth_role ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let path_id: Uuid = match row.try_get(2) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get path_id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let path: String = match row.try_get(3) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get path ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let readable_path: String = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get readable_path ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let get_allowed: bool = match row.try_get(5) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get get_allowed ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let post_allowed: bool = match row.try_get(6) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get post_allowed ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let put_allowed: bool = match row.try_get(7) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get put_allowed ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let delete_allowed: bool = match row.try_get(8) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get delete_allowed ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_delegate_get: bool = match row.try_get(9) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_delegate_get ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_delegate_post: bool = match row.try_get(10) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_delegate_post ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_delegate_put: bool = match row.try_get(11) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_delegate_put ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_delegate_delete: bool = match row.try_get(12) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_delegate_delete ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_access_for_children_get: bool = match row.try_get(13) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_access_for_children_get ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_access_for_children_post: bool = match row.try_get(14) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_access_for_children_post ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_access_for_children_put: bool = match row.try_get(15) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_access_for_children_put ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let can_access_for_children_delete: bool = match row.try_get(16) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_access_for_children_delete ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let where_replacement: Option<String> = match row.try_get(17) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(18) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get created ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(19) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let res = AuthRolesCrossPaths {
                id: id.to_hyphenated().to_string(),
                auth_role: Some(auth_role.to_hyphenated().to_string()),
                path_id: Some(path_id.to_hyphenated().to_string()),
                path,
                readable_path,
                get_allowed,
                post_allowed,
                put_allowed,
                delete_allowed,
                can_delegate_get,
                can_delegate_post,
                can_delegate_put,
                can_delegate_delete,
                can_access_for_children_get,
                can_access_for_children_post,
                can_access_for_children_put,
                can_access_for_children_delete,
                where_replacement,
                created: Some(created),
                modified,
            };

            results_to_send.push(res);
        }

        Ok(results_to_send)
    }
}

#[async_trait]
impl AuthRolesCrossPathsContracts for AuthRolesCrossPaths {
    async fn find_auth_roles_cross_paths_for_role_id(role_id: &str, db_pool: &DbPool)
                                                     -> Result<Vec<AuthRolesCrossPaths>, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM auth_roles_cross_paths WHERE auth_role = '{}'", role_id);

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results_to_send = match AuthRolesCrossPaths::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if results_to_send.len() != 0 {
            Ok(
                results_to_send
            )
        } else {
            StatusMessage::bad_request_400_in_result(
                "Failed to get auth_roles_cross_paths".to_owned()
            )
        }
    }
}