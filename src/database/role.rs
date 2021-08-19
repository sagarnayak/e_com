use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use postgres::Row;
use rocket::State;
use uuid::Uuid;

use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::table_rows_count_contracts::TableRowsCountContracts;
use crate::core::postgres_error_master::get_postgres_error_string;
use crate::database::database_master::resolve_client;
use crate::database::db_pool::DbPool;
use crate::model::page_response::PageResponse;
use crate::model::role::Role;
use crate::model::role_request::RoleRequest;
use crate::model::status_message::StatusMessage;
use crate::model::table_rows_count::TableRowsCount;
use crate::model::user::User;

impl Role {
    async fn convert_results_to_models(results: &Vec<Row>) -> Result<Vec<Role>, StatusMessage> {
        let mut results_to_send: Vec<Role> = vec![];

        for row in results {
            let id: Uuid = match row.try_get(0) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get id ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let derived_from: Option<Uuid> = match row.try_get(1) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
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
            let can_access_for_children: bool = match row.try_get(4) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get can_access_for_children ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let enabled: bool = match row.try_get(5) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get enabled ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let valid_from: Option<DateTime<Utc>> = match row.try_get(6) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let valid_to: Option<DateTime<Utc>> = match row.try_get(7) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let created: DateTime<Utc> = match row.try_get(8) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => return StatusMessage::bad_request_400_in_result("failed to get enabled ".to_string()),
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };
            let modified: Option<DateTime<Utc>> = match row.try_get(9) {
                Ok(positive) => match positive {
                    Some(positive_inner) => positive_inner,
                    None => None,
                },
                Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
            };

            let role = Role {
                id: id.to_hyphenated().to_string(),
                derived_from:
                if derived_from.is_some() {
                    Some(derived_from.unwrap().to_hyphenated().to_string())
                } else {
                    None
                },
                name,
                can_delegate,
                can_access_for_children,
                enabled,
                valid_from,
                valid_to,
                created,
                modified,
            };

            results_to_send.push(role);
        }

        Ok(results_to_send)
    }

    #[async_recursion]
    pub async fn if_role_is_direct_or_indirect_parent(
        anticipated_parent_role: &Role,
        anticipated_child_role: &Role,
        db_pool: &State<DbPool>,
    )
        -> Result<bool, StatusMessage> {
        let anticipated_child_role_derived_from = match anticipated_child_role.clone().derived_from {
            Some(positive) => {
                positive
            }
            None => {
                return StatusMessage::bad_request_400_in_result(
                    "Child does not have parent role id".to_owned()
                );
            }
        };

        if anticipated_child_role_derived_from == anticipated_parent_role.id {
            return Ok(true);
        } else {
            let immediate_parent_role_id = match anticipated_child_role.clone().derived_from {
                Some(positive) => {
                    positive
                }
                None => {
                    return StatusMessage::bad_request_400_in_result(
                        "failed to get derived role id".to_owned()
                    );
                }
            };

            let immediate_parent_role = match Role::find_role_for_role_id(
                &immediate_parent_role_id,
                &db_pool,
            ).await {
                Ok(positive) => {
                    positive
                }
                Err(error) => {
                    return StatusMessage::bad_request_400_in_result(
                        error.message
                    );
                }
            };

            Role::if_role_is_direct_or_indirect_parent(
                &immediate_parent_role,
                &anticipated_child_role,
                &db_pool,
            ).await
        }
    }
}

#[async_trait]
impl RoleContracts for Role {
    async fn find_role_for(user: &User, db_pool: &State<DbPool>) -> Result<Role, StatusMessage> {
        Role::find_role_for_role_id(
            &user.role,
            &db_pool,
        ).await
    }

    async fn find_role_for_role_id(role_id: &str, db_pool: &State<DbPool>) -> Result<Role, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM roles WHERE id = '{}'", &role_id);

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

        let results_to_send = match Role::convert_results_to_models(&results).await {
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
            StatusMessage::bad_request_400_in_result(
                "Failed to get role".to_owned()
            )
        }
    }

    async fn find_role_for_admin(db_pool: &DbPool) -> Result<Role, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!("SELECT * FROM roles WHERE name = 'genesis'");

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

        let roles = match Role::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        if roles.len() != 0 {
            let role_to_return = roles[0].clone();
            Ok(
                role_to_return
            )
        } else {
            StatusMessage::bad_request_400_in_result(
                "Failed to get role".to_owned()
            )
        }
    }

    async fn add_role(user_role: &Role, role: &RoleRequest, db_pool: &DbPool) -> Result<Uuid, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let uuid_for_role = Uuid::new_v4();

        let mut columns_statement = "INSERT INTO roles (id,derived_from,name,can_delegate,enabled".to_owned();

        if role.valid_from.is_some() {
            columns_statement.push_str(",valid_from");
        }
        if role.valid_to.is_some() {
            columns_statement.push_str(",valid_to");
        }
        if role.can_access_for_children.is_some() {
            columns_statement.push_str(",can_access_for_children");
        }
        columns_statement.push_str(") ");

        let mut values_statement = format!(
            "VALUES ('{}','{}','{}',{},{}",
            &uuid_for_role.to_hyphenated().to_string(),
            &user_role.id,
            &role.name,
            &role.can_delegate,
            &role.enabled
        );

        if role.valid_from.is_some() {
            values_statement.push_str(&format!(",'{}'", role.valid_from.unwrap()))
        }
        if role.valid_to.is_some() {
            values_statement.push_str(&format!(",'{}'", role.valid_to.unwrap()))
        }
        if role.can_access_for_children.is_some() {
            values_statement.push_str(&format!(",{}", role.can_access_for_children.unwrap()))
        }
        values_statement.push_str(")");

        let statement_to_send = &format!(
            "{} {}", columns_statement, values_statement
        );

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.execute(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(
                    get_postgres_error_string(error.as_db_error())
                );
            }
        };

        if results != 0 {
            Ok(
                uuid_for_role
            )
        } else {
            StatusMessage::bad_request_400_in_result(
                "Failed to add role".to_owned()
            )
        }
    }

    async fn find_roles_created_by_role(
        role: &Role,
        page_number: &u32,
        page_size: &u32,
        db_pool: &State<DbPool>,
    ) -> Result<PageResponse<Role>, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let offset = if page_number.eq(&1) {
            0
        } else {
            page_size * page_number
        };

        let statement_to_send = &format!(
            "SELECT * FROM roles WHERE derived_from = '{}' LIMIT {} OFFSET {}",
            &role.id,
            &page_size,
            &offset,
        );

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

        let roles = match Role::convert_results_to_models(&results).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return Err(error);
            }
        };

        let where_condition_string =
            format!("derived_from = '{}'", &role.id);

        let total_count = match TableRowsCount::find_count(
            "roles",
            &Some(where_condition_string),
            &db_pool,
        ).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(
                    error.message
                );
            }
        };

        let first_page: u32 = 1;

        Ok(
            PageResponse {
                data: roles,
                previous_page_available: page_number.clone() != first_page,
                next_page_available: (page_size * page_number) < total_count,
                page_size: page_size.clone(),
                page_number: page_number.clone(),
                total_items: total_count,
            }
        )
    }

    async fn if_role_created_by(
        role_id_to_check: &String,
        parent_role_id: &String,
        db_pool: &State<DbPool>,
    )
        -> Result<bool, StatusMessage> {
        let client = resolve_client(db_pool).await;

        let statement_to_send = &format!(
            "SELECT * FROM roles WHERE id = '{}' AND derived_from = '{}'",
            &role_id_to_check,
            &parent_role_id
        );

        let statement = match client
            .prepare_cached(statement_to_send)
            .await {
            Ok(statement_positive) => statement_positive,
            Err(error) => return StatusMessage::bad_request_400_in_result(error.to_string()),
        };

        let results = match client.query(&statement, &[]).await {
            Ok(result_positive) => result_positive,
            Err(error) => {
                return StatusMessage::bad_request_400_in_result(
                    get_postgres_error_string(error.as_db_error())
                );
            }
        };

        if results.len() != 0 {
            Ok(
                true
            )
        } else {
            Ok(
                false
            )
        }
    }
}