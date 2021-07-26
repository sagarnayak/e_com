use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::config_controller::ConfigData;
use crate::core::strings::{BAD_REQUEST, UNAUTHORIZED, WELCOME_ADMIN};
use crate::model::authentication_request::AuthenticationRequest;
use crate::model::authentication_response::AuthenticationResponse;
use crate::model::status_message::StatusMessage;

#[post("/authenticate", data = "<authentication_request>")]
pub fn authenticate(authentication_request: Option<Json<AuthenticationRequest>>)
                    -> status::Custom<Result<Json<AuthenticationResponse>, Json<StatusMessage>>> {
    let authentication_request = match authentication_request {
        Some(positive) => positive,
        None => return StatusMessage::bad_request_400_with_status_code_in_result(
            BAD_REQUEST.to_string()
        ),
    };

    let config_data = ConfigData::new();
    if authentication_request.user_name == config_data.admin_data.admin_name {
        return if authentication_request.password == config_data.admin_data.admin_password {
            StatusMessage::ok_200_with_status_code_in_result(
                WELCOME_ADMIN.to_string()
            )
        } else {
            StatusMessage::unauthorized_401_with_status_code_in_result(
                UNAUTHORIZED.to_string()
            )
        }
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