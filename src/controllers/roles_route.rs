use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

use crate::config_controller::ConfigData;
use crate::contracts::auth_roles_cross_paths_contracts::AuthRolesCrossPathsContracts;
use crate::contracts::path_contracts::PathContracts;
use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::table_rows_count_contracts::TableRowsCountContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::controllers::auth_roles_cross_paths_route::add_auth_roles_cross_paths;
use crate::core::strings::{BAD_REQUEST, FORBIDDEN};
use crate::database::db_pool::DbPool;
use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::page_response::PageResponse;
use crate::model::path::Path;
use crate::model::role::Role;
use crate::model::role_request::RoleRequest;
use crate::model::status_message::StatusMessage;
use crate::model::table_rows_count::TableRowsCount;
use crate::model::user::User;

#[get("/role")]
pub async fn get_my_role(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
)
    -> status::Custom<Result<Json<Role>, Json<StatusMessage>>> {
    let authentication_authorization_guard = match authentication_authorization_guard {
        Ok(positive) => { positive }
        Err(error) => {
            return StatusMessage::dynamic_error_with_status_code_in_result(error);
        }
    };

    let user = match User::find_user_with_id(
        &authentication_authorization_guard.claims.owner,
        db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let role = match Role::find_role_for(&user, db_pool).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    status::Custom(
        Status::Ok,
        Ok(
            Json(
                role
            )
        ),
    )
}

#[post("/role", data = "<role_request>")]
pub async fn create_role(
    role_request: Option<Json<RoleRequest>>,
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
)
    -> status::Custom<Result<Json<StatusMessage>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(error);
            }
        };

    let user = match User::find_user_with_id(
        &authentication_authorization_guard.claims.owner,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let user_own_role = match Role::find_role_for(
        &user,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    if !&user_own_role.can_delegate {
        return StatusMessage::forbidden_403_with_status_code_in_result(
            FORBIDDEN.to_string()
        );
    }

    let role_request = match role_request {
        Some(positive) => positive,
        None => return StatusMessage::bad_request_400_with_status_code_in_result(
            BAD_REQUEST.to_string()
        ),
    };

    let uuid_of_inserted_role: Uuid;

    let add_role_result = match Role::add_role(&user_own_role, &role_request, &db_pool).await {
        Ok(positive) => {
            uuid_of_inserted_role = positive;
            let _ = TableRowsCount::clear_data_for_table(
                "roles",
                &db_pool,
            ).await;
            StatusMessage::ok_200_with_status_code_in_result(
                "Role is created".to_owned()
            )
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let path_authorised_for_user = match AuthRolesCrossPaths::find_auth_roles_cross_paths_for_role_id(
        &user_own_role.id,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let paths_from_master_table = match Path::fetch_all(
        &db_pool
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    for auth_path in path_authorised_for_user.iter() {
        let auth_path_id = auth_path.clone().path_id.unwrap();
        for master_path in paths_from_master_table.iter() {
            let master_path_id = master_path.clone().id.unwrap();
            if master_path_id == auth_path_id &&
                (
                    master_path.force_delegate_get ||
                        master_path.force_delegate_post ||
                        master_path.force_delegate_put ||
                        master_path.force_delegate_delete
                ) {
                let result_of_auth_path_insert = match AuthRolesCrossPaths::add_auth_roles_cross_paths(
                    &uuid_of_inserted_role,
                    &master_path,
                    &db_pool,
                ).await {
                    Ok(positive) => {
                        positive
                    }
                    Err(error) => {
                        return StatusMessage::bad_request_400_with_status_code_in_result(
                            error.message
                        );
                    }
                };
            }
        }
    }

    add_role_result
}

#[get("/roles?<page>&<size>")]
pub async fn find_roles_created_by_me(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
    config_data: &State<ConfigData>,
    page: Option<u32>,
    size: Option<u32>,
)
    -> status::Custom<Result<Json<PageResponse<Role>>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(error);
            }
        };

    let user = match User::find_user_with_id(
        &authentication_authorization_guard.claims.owner,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let user_own_role = match Role::find_role_for(
        &user,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let page_number = match page {
        Some(positive) => {
            positive
        }
        None => {
            1
        }
    };

    let page_size = match size {
        Some(positive) => {
            if positive <= config_data.paging_conf.max_page_size {
                positive
            } else {
                config_data.paging_conf.max_page_size
            }
        }
        None => {
            config_data.paging_conf.default_page_size
        }
    };

    return match Role::find_roles_created_by_role(
        &user_own_role,
        &page_number,
        &page_size,
        &db_pool,
    ).await {
        Ok(positive) => {
            status::Custom(
                Status::Ok,
                Ok(
                    Json(
                        positive
                    )
                ),
            )
        }
        Err(error) => {
            StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            )
        }
    };
}

#[get("/roles/<role_creator_role_id>?<page>&<size>")]
pub async fn find_roles_created_by_specific_user(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
    config_data: &State<ConfigData>,
    role_creator_role_id: Option<String>,
    page: Option<u32>,
    size: Option<u32>,
)
    -> status::Custom<Result<Json<PageResponse<Role>>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(error);
            }
        };

    let user = match User::find_user_with_id(
        &authentication_authorization_guard.claims.owner,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let user_own_role = match Role::find_role_for(
        &user,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let role_creator_role_id = match role_creator_role_id {
        Some(positive) => {
            positive
        }
        None => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                "Please provide role id".to_owned()
            );
        }
    };

    let role_data_of_role_creator = match Role::find_role_for_role_id(
        &role_creator_role_id,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(_) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                "Role not found".to_owned()
            );
        }
    };

    let if_allowed_to_view_roles = match Role::if_role_is_direct_or_indirect_parent(
        &user_own_role,
        &role_data_of_role_creator,
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::forbidden_403_with_status_code_in_result(
                error.message
            );
        }
    };

    if !if_allowed_to_view_roles {
        return StatusMessage::forbidden_403_with_status_code_in_result(
            "You are not allowed to view roles created by this role".to_owned()
        );
    }

    let page_number = match page {
        Some(positive) => {
            positive
        }
        None => {
            1
        }
    };

    let page_size = match size {
        Some(positive) => {
            if positive <= config_data.paging_conf.max_page_size {
                positive
            } else {
                config_data.paging_conf.max_page_size
            }
        }
        None => {
            config_data.paging_conf.default_page_size
        }
    };

    return match Role::find_roles_created_by_role(
        &role_data_of_role_creator,
        &page_number,
        &page_size,
        &db_pool,
    ).await {
        Ok(positive) => {
            status::Custom(
                Status::Ok,
                Ok(
                    Json(
                        positive
                    )
                ),
            )
        }
        Err(error) => {
            StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            )
        }
    };
}