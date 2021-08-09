use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config_controller::ConfigData;
use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::table_rows_count_contracts::TableRowsCountContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::core::strings::{BAD_REQUEST, FORBIDDEN};
use crate::database::db_pool::DbPool;
use crate::guards::authentication_guard::AuthenticationGuard;
use crate::guards::authorization_guard::AuthorizationGuard;
use crate::model::page_response::PageResponse;
use crate::model::role::Role;
use crate::model::role_request::RoleRequest;
use crate::model::status_message::StatusMessage;
use crate::model::table_rows_count::TableRowsCount;
use crate::model::user::User;

#[get("/role")]
pub async fn get_my_role(
    authentication_guard: Result<AuthenticationGuard, StatusMessage>,
    authorization_guard: Result<AuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
)
    -> status::Custom<Result<Json<Role>, Json<StatusMessage>>> {
    let authentication_guard = match authentication_guard {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                error.message
            );
        }
    };

    match authorization_guard {
        Ok(_) => {}
        Err(error) => {
            return StatusMessage::forbidden_403_with_status_code_in_result(
                error.message
            );
        }
    }

    let user = match User::find_user_with_id(authentication_guard.claims.owner.clone(), db_pool).await {
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
    authentication_guard: Result<AuthenticationGuard, StatusMessage>,
    authorization_guard: Result<AuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
)
    -> status::Custom<Result<Json<StatusMessage>, Json<StatusMessage>>> {
    let authentication_guard = match authentication_guard {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                error.message
            );
        }
    };

    match authorization_guard {
        Ok(_) => {}
        Err(error) => {
            return StatusMessage::forbidden_403_with_status_code_in_result(
                error.message
            );
        }
    }

    let user = match User::find_user_with_id(
        authentication_guard.claims.owner,
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

    return match Role::add_role(&user_own_role, &role_request, &db_pool).await {
        Ok(_) => {
            let _ = TableRowsCount::clear_data_for_table(
                "roles",
                &db_pool,
            ).await;
            StatusMessage::ok_200_with_status_code_in_result(
                "Role is created".to_owned()
            )
        }
        Err(error) => {
            StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            )
        }
    };
}

#[get("/roles?<page>&<size>")]
pub async fn find_roles_created_by_me(
    authentication_guard: Result<AuthenticationGuard, StatusMessage>,
    authorization_guard: Result<AuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
    page: Option<u32>,
    size: Option<u32>,
)
    -> status::Custom<Result<Json<PageResponse<Role>>, Json<StatusMessage>>> {
    let authentication_guard = match authentication_guard {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                error.message
            );
        }
    };

    match authorization_guard {
        Ok(_) => {}
        Err(error) => {
            return StatusMessage::forbidden_403_with_status_code_in_result(
                error.message
            );
        }
    }

    let user = match User::find_user_with_id(
        authentication_guard.claims.owner,
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

    let conf_data = ConfigData::new();

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
            if positive <= conf_data.paging_conf.max_page_size {
                positive
            } else {
                conf_data.paging_conf.max_page_size
            }
        }
        None => {
            conf_data.paging_conf.default_page_size
        }
    };

    return match Role::find_roles_created_by_me(
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