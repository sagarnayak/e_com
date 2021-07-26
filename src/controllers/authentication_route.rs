use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::core::strings::BAD_REQUEST;
use crate::model::authentication_request::AuthenticationRequest;
use crate::model::authentication_response::AuthenticationResponse;
use crate::model::status_message::StatusMessage;

#[post("/authenticate", data = "<authentication_request>")]
pub fn authenticate(authentication_request: Option<Json<AuthenticationRequest>>)
                    -> status::Custom<Result<Json<AuthenticationResponse>, Json<StatusMessage>>> {
    let _ = match authentication_request {
        Some(positive) => positive,
        None => return StatusMessage::bad_request_400_with_status_code_in_result(
            BAD_REQUEST.to_string()
        ),
    };
    status::Custom(
        Status::Ok,
        Ok(Json(
            AuthenticationResponse {
                jwt: "sagar".to_string(),
            }
        )),
    )
}