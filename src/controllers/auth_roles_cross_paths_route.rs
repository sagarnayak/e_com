use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::status_message::StatusMessage;

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