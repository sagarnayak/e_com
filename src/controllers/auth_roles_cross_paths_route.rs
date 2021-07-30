use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::guards::authentication_guard::AuthenticationGuard;
use crate::guards::authorization_guard::AuthorizationGuard;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::status_message::StatusMessage;

#[get("/paths")]
pub async fn get_available_paths(
    authentication_guard: Result<AuthenticationGuard, StatusMessage>,
    authorization_guard: Result<AuthorizationGuard, StatusMessage>,
)
    -> status::Custom<Result<Json<Vec<AuthRolesCrossPaths>>, Json<StatusMessage>>> {
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

    status::Custom(
        Status::Ok,
        Ok(
            Json(
                authentication_guard.claims.authorizations
            )
        ),
    )
}