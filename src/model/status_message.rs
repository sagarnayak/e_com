use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub code: u16,
    pub message: String,
}

impl StatusMessage {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // customizable
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn custom(message: String, status: Status) -> StatusMessage {
        StatusMessage {
            code: status.code,
            message,
        }
    }
    pub fn custom_with_status_code(message: String, status: Status)
                                   -> status::Custom<Json<StatusMessage>> {
        status::Custom(
            status,
            Json(
                StatusMessage::custom(message, status)
            ),
        )
    }
    pub fn custom_with_status_code_in_result<T>(message: String, status: Status)
                                                -> status::Custom<Result<T, Json<StatusMessage>>> {
        status::Custom(
            status,
            Err(
                Json(
                    StatusMessage::custom(message, status)
                )
            ),
        )
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // derived methods
    ////////////////////////////////////////////////////////////////////////////////////////////////

    // for 200
    pub fn ok_200_in_result<T>(message: String) -> Result<StatusMessage, T> {
        Ok(
            StatusMessage::custom(
                message,
                Status::Ok,
            )
        )
    }
    pub fn ok_200_with_status_code(message: String)
                                   -> status::Custom<Json<StatusMessage>> {
        StatusMessage::custom_with_status_code(
            message,
            Status::Ok,
        )
    }
    pub fn ok_200_with_status_code_in_result<T>(message: String)
                                                -> status::Custom<Result<Json<StatusMessage>, T>> {
        status::Custom(
            Status::Ok,
            Ok(
                Json(
                    StatusMessage::custom(message, Status::Ok)
                )
            ),
        )
    }

    // for 400
    pub fn bad_request_400_in_result<T>(message: String) -> Result<T, StatusMessage> {
        Err(
            StatusMessage::custom(
                message,
                Status::BadRequest,
            )
        )
    }
    pub fn bad_request_400_with_status_code(message: String)
                                            -> status::Custom<Json<StatusMessage>> {
        StatusMessage::custom_with_status_code(
            message,
            Status::BadRequest,
        )
    }
    pub fn bad_request_400_with_status_code_in_result<T>(message: String)
                                                         -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::BadRequest,
        )
    }

    // for 401
    pub fn unauthorized_401_in_result<T>(message: String) -> Result<T, StatusMessage> {
        Err(
            StatusMessage::custom(
                message,
                Status::Unauthorized,
            )
        )
    }
    pub fn unauthorized_401_with_status_code(message: String)
                                             -> status::Custom<Json<StatusMessage>> {
        StatusMessage::custom_with_status_code(
            message,
            Status::Unauthorized,
        )
    }
    pub fn unauthorized_401_with_status_code_in_result<T>(message: String)
                                                          -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::Unauthorized,
        )
    }

    // for 404
    pub fn not_found_404_in_result<T>(message: String) -> Result<T, StatusMessage> {
        Err(
            StatusMessage::custom(
                message,
                Status::NotFound,
            )
        )
    }
    pub fn not_found_404_with_status_code(message: String)
                                          -> status::Custom<Json<StatusMessage>> {
        StatusMessage::custom_with_status_code(
            message,
            Status::NotFound,
        )
    }
    pub fn not_found_404_with_status_code_in_result<T>(message: String)
                                                       -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::NotFound,
        )
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
}