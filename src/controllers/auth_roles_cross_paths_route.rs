use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::database::db_pool::DbPool;
use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::auth_roles_cross_paths_request::AuthRolesCrossPathsRequest;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/paths")]
pub async fn get_available_paths(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
)
    -> status::Custom<Result<Json<Vec<AuthRolesCrossPaths>>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(
                    error
                );
            }
        };

    status::Custom(
        Status::Ok,
        Ok(
            Json(
                authentication_authorization_guard.auth_expanded
            )
        ),
    )
}

#[post("/paths", data = "<auth_roles_cross_path_request>")]
pub async fn add_auth_roles_cross_paths(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
    auth_roles_cross_path_request: Option<Json<AuthRolesCrossPathsRequest>>,
)
    -> status::Custom<Result<Json<Vec<AuthRolesCrossPaths>>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(
                    error
                );
            }
        };

    let auth_roles_cross_path_request = match auth_roles_cross_path_request {
        Some(positive) => { positive }
        None => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                "Please provide auth roles cross paths request.".to_owned()
            );
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

    let parent_role = match Role::find_role_for(
        &user,
        &db_pool,
    ).await {
        Ok(positive) => positive,
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let if_role_created_by_user = match Role::if_role_created_by(
        &auth_roles_cross_path_request.role_id,
        &parent_role.id,
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

    if !if_role_created_by_user {
        return StatusMessage::bad_request_400_with_status_code_in_result(
            "The role is not created by you.".to_owned()
        );
    }

    for auth_request in auth_roles_cross_path_request.paths.iter() {
        println!("requested paths : {}", auth_request.path_id);
    }

    for user_authorised_path in authentication_authorization_guard.auth_expanded.iter() {
        println!("user auth >> {} {}", user_authorised_path.path, user_authorised_path.id);
    }

    status::Custom(
        Status::Ok,
        Ok(
            Json(
                authentication_authorization_guard.auth_expanded
            )
        ),
    )
}