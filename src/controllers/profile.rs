use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::user_contracts::UserContracts;
use crate::database::db_pool::DbPool;
use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/me")]
pub async fn me(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
) -> status::Custom<Result<Json<User>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(error);
            }
        };

    let user = match User::find_user_with_id(
        authentication_authorization_guard.claims.owner.clone(),
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

    StatusMessage::ok_200_with_status_code_in_result_generic(user)
}