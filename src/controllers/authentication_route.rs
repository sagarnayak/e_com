use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config_controller::ConfigData;
use crate::contracts::role_contracts::RoleContracts;
use crate::core::strings::{BAD_REQUEST, UNAUTHORIZED, WELCOME_ADMIN};
use crate::database::db_pool::DbPool;
use crate::guards::authentication_guard::AuthenticationGuard;
use crate::jwt_master::jwt_master::create_jwt;
use crate::model::authentication_request::AuthenticationRequest;
use crate::model::authentication_response::AuthenticationResponse;
use crate::model::role::Role;
use crate::model::status_message::StatusMessage;

#[post("/authenticate", data = "<authentication_request>")]
pub async fn authenticate(
    authentication_request: Option<Json<AuthenticationRequest>>,
    db_pool: &State<DbPool>
)
    -> status::Custom<Result<Json<AuthenticationResponse>, Json<StatusMessage>>> {
    let authentication_request = match authentication_request {
        Some(positive) => positive,
        None => return StatusMessage::bad_request_400_with_status_code_in_result(
            BAD_REQUEST.to_string()
        ),
    };

    // authentication_guard: Result<AuthenticationGuard, StatusMessage>,
    //
    // match authentication_guard {
    //     Ok(positive) => {
    //         println!("Good to go {:?}", positive);
    //     }
    //     Err(error) => {
    //         println!("Got an error {:?}", error);
    //     }
    // }

    let config_data = ConfigData::new();
    if authentication_request.user_name == config_data.admin_data.admin_name {
        return if authentication_request.password == config_data.admin_data.admin_password {
            let role = Role::find_role_for(db_pool).await;
            match create_jwt(
                60,
                config_data.admin_data.admin_name.to_string(),
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
        } else {
            StatusMessage::unauthorized_401_with_status_code_in_result(
                UNAUTHORIZED.to_string()
            )
        };
    }

    status::Custom(
        Status::Ok,
        Ok(Json(
            AuthenticationResponse {
                jwt: "working on it".to_string(),
            }
        )),
    )
}