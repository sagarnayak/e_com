use bcrypt::verify;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::auth_roles_cross_paths_contracts::AuthRolesCrossPathsContracts;
use crate::contracts::blocked_for_platform_authorization_contracts::BlockedForPlatformAuthorizationContracts;
use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::core::constants::NEED_PLATFORM_AUTH;
use crate::core::strings::{AUTHENTICATION_FAILURE, BAD_REQUEST};
use crate::database::db_pool::DbPool;
use crate::jwt_master::jwt_master::create_jwt;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::authentication_request::AuthenticationRequest;
use crate::model::authentication_response::AuthenticationResponse;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[post("/authenticate", data = "<authentication_request>")]
pub async fn authenticate(
    authentication_request: Option<Json<AuthenticationRequest>>,
    db_pool: &State<DbPool>,
)
    -> status::Custom<Result<Json<AuthenticationResponse>, Json<StatusMessage>>> {
    let authentication_request = match authentication_request {
        Some(positive) => positive,
        None => return StatusMessage::bad_request_400_with_status_code_in_result(
            BAD_REQUEST.to_string()
        ),
    };

    let user: User = match User::find_user_with_email(
        authentication_request.user_email.clone(),
        db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                error.message,
                None,
                None,
            );
        }
    };

    match verify(
        &authentication_request.password,
        &user.password,
    ) {
        Ok(positive) => {
            if !positive {
                return StatusMessage::unauthorized_401_with_status_code_in_result(
                    AUTHENTICATION_FAILURE.to_string(),
                    None,
                    None,
                );
            }
        }
        Err(_) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                AUTHENTICATION_FAILURE.to_string(),
                None,
                None,
            );
        }
    }

    let _ = match BlockedForPlatformAuthorization::find_data_with_user_id(
        &user.id,
        &db_pool,
    ).await {
        Ok(_) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                "Please perform a platform authorization on your previous \
                logged in device or contact admin".to_owned(),
                Some(NEED_PLATFORM_AUTH),
                None,
            );
        }
        Err(_) => {}
    };

    let role: Role = match Role::find_role_for(
        &user,
        db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(
                error.message,
                None,
                None,
            );
        }
    };

    let auth_roles_cross_paths: Vec<AuthRolesCrossPaths> =
        match AuthRolesCrossPaths::find_auth_roles_cross_paths_for_role_id(
            &role.id,
            db_pool,
        ).await {
            Ok(positive) => {
                positive
            }
            Err(error) => {
                return StatusMessage::unauthorized_401_with_status_code_in_result(
                    error.message,
                    None,
                    None,
                );
            }
        };

    match create_jwt(
        60 * 60,
        60 * 60 * 24 * 30,
        &user,
        auth_roles_cross_paths,
        &db_pool,
    ).await {
        Ok(positive) => status::Custom(
            Status::Ok,
            Ok(Json(AuthenticationResponse { jwt: positive.0, refresh_token: positive.1 })),
        ),
        Err(error) => status::Custom(
            Status::BadRequest,
            Err(Json(error)),
        )
    }
}