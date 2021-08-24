use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub code: u16,
    #[serde(skip_serializing, skip_deserializing)]
    pub status: Status,
    pub message: String,
    pub sys_message: Option<String>,
}

impl StatusMessage {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // customizable
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn custom(message: String, status: Status, code: Option<u16>, sys_message: Option<String>) -> StatusMessage {
        StatusMessage {
            code: if code.is_some() { code.unwrap() } else { status.code },
            status,
            message,
            sys_message,
        }
    }
    pub fn custom_with_status_code(message: String, status: Status)
                                   -> status::Custom<Json<StatusMessage>> {
        status::Custom(
            status,
            Json(
                StatusMessage::custom(message, status, None, None)
            ),
        )
    }
    pub fn custom_with_status_code_in_result<T>(
        message: String,
        status: Status,
        code: Option<u16>,
        sys_message: Option<String>,
    )
        -> status::Custom<Result<T, Json<StatusMessage>>> {
        status::Custom(
            status,
            Err(
                Json(
                    StatusMessage::custom(
                        message,
                        status,
                        code,
                        sys_message,
                    )
                )
            ),
        )
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // models to copy
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // pub fn bad_request_400_in_result<T>(message: String) -> Result<T, StatusMessage> {
    //     Err(
    //         StatusMessage::custom(
    //             message,
    //             Status::BadRequest,
    //         )
    //     )
    // }
    // pub fn bad_request_400_with_status_code(message: String)
    //                                         -> status::Custom<Json<StatusMessage>> {
    //     StatusMessage::custom_with_status_code(
    //         message,
    //         Status::BadRequest,
    //     )
    // }
    // pub fn bad_request_400_with_status_code_in_result<T>(message: String)
    //                                                      -> status::Custom<Result<T, Json<StatusMessage>>> {
    //     StatusMessage::custom_with_status_code_in_result(
    //         message,
    //         Status::BadRequest,
    //     )
    // }

    // for 200
    // pub fn ok_200_in_result<T>(message: String) -> Result<StatusMessage, T> {
    //     Ok(
    //         StatusMessage::custom(
    //             message,
    //             Status::Ok,
    //         )
    //     )
    // }
    // pub fn ok_200_with_status_code(message: String)
    //                                -> status::Custom<Json<StatusMessage>> {
    //     StatusMessage::custom_with_status_code(
    //         message,
    //         Status::Ok,
    //     )
    // }
    // pub fn ok_200_with_status_code_in_result<T>(message: String)
    //                                             -> status::Custom<Result<Json<StatusMessage>, T>> {
    //     status::Custom(
    //         Status::Ok,
    //         Ok(
    //             Json(
    //                 StatusMessage::custom(message, Status::Ok)
    //             )
    //         ),
    //     )
    // }
    ////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // derived methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    //for dynamic types
    pub fn dynamic_error_with_status_code_in_result<T>(status_message: StatusMessage)
                                                       -> status::Custom<Result<Json<T>, Json<StatusMessage>>> {
        status::Custom(
            status_message.status,
            Err(
                Json(
                    status_message
                )
            ),
        )
    }

    //for 200
    pub fn ok_200_with_status_code_in_result<T>(message: String)
                                                -> status::Custom<Result<Json<StatusMessage>, T>> {
        status::Custom(
            Status::Ok,
            Ok(
                Json(
                    StatusMessage::custom(message, Status::Ok, None, None)
                )
            ),
        )
    }
    pub fn ok_200_with_status_code_in_result_generic<T>(data: T)
                                                        -> status::Custom<Result<Json<T>, Json<StatusMessage>>> {
        status::Custom(
            Status::Ok,
            Ok(
                Json(
                    data
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
                None,
                None,
            )
        )
    }
    pub fn bad_request_400_with_status_code_in_result<T>(message: String)
                                                         -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::BadRequest,
            None,
            None,
        )
    }

    //for 403
    pub fn forbidden_403_with_status_code_in_result<T>(message: String)
                                                       -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::Forbidden,
            None,
            None,
        )
    }

    // for 404
    pub fn not_found_404_with_status_code(message: String)
                                          -> status::Custom<Json<StatusMessage>> {
        StatusMessage::custom_with_status_code(
            message,
            Status::NotFound,
        )
    }
    pub fn unauthorized_401_with_status_code_in_result<T>(message: String, code: Option<u16>, sys_message: Option<String>)
                                                          -> status::Custom<Result<T, Json<StatusMessage>>> {
        StatusMessage::custom_with_status_code_in_result(
            message,
            Status::Unauthorized,
            code,
            sys_message,
        )
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
}