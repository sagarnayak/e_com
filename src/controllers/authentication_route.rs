use rocket::http::Status;
use rocket::response::status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;

use crate::config_controller::ConfigData;
use crate::contracts::role_contracts::RoleContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::core::strings::{BAD_REQUEST, UNAUTHORIZED};
use crate::database::db_pool::DbPool;
use crate::jwt_master::jwt_master::create_jwt;
use crate::model::authentication_request::AuthenticationRequest;
use crate::model::authentication_response::AuthenticationResponse;
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
            return StatusMessage::unauthorized_401_with_status_code_in_result(error.message);
        }
    };

    let role: Role = match Role::find_role_for(
        &user,
        db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::unauthorized_401_with_status_code_in_result(error.message);
        }
    };

    match create_jwt(
        60,
        &user,
        &role,
    ) {
        Ok(positive) => status::Custom(
            Status::Ok,
            Ok(Json(AuthenticationResponse { jwt: positive })),
        ),
        Err(error) => status::Custom(
            Status::BadRequest,
            Err(Json(error)),
        )
    }
}