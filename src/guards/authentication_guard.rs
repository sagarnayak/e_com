use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::core::strings::{AUTHENTICATION_FAILURE, EXPIRED_AUTH_TOKEN};
use crate::jwt_master::jwt_master::{extract_jwt, validate_jwt};
use crate::model::claims::Claims;
use crate::model::status_message::StatusMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticationGuard {
    type Error = StatusMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, StatusMessage> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> (bool, Option<bool>, String) {
            let if_bearer_token_found: bool = key.contains("Bearer ");
            if !if_bearer_token_found {
                return (false, None, "".to_owned());
            }

            let bearer_replaced = key.replace("Bearer ", "");

            let validated_result = validate_jwt(&bearer_replaced);

            (validated_result.0, Some(validated_result.1), bearer_replaced.to_owned())
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, StatusMessage {
                code: 401,
                status: Status::Unauthorized,
                message: AUTHENTICATION_FAILURE.to_string(),
            })),
            Some(key) => {
                let validated_result = is_valid(key);
                if validated_result.0 {
                    match extract_jwt(&validated_result.2) {
                        Ok(claims) => {
                            Outcome::Success(
                                AuthenticationGuard {
                                    claims
                                }
                            )
                        }
                        Err(status_message) => {
                            Outcome::Failure(
                                (
                                    Status::Unauthorized,
                                    StatusMessage {
                                        code: 401,
                                        status: Status::Unauthorized,
                                        message: status_message.message,
                                    }
                                )
                            )
                        }
                    }
                } else {
                    match validated_result.1 {
                        Some(is_expired) => {
                            if is_expired {
                                Outcome::Failure(
                                    (
                                        Status::Unauthorized,
                                        StatusMessage {
                                            code: 401,
                                            status: Status::Unauthorized,
                                            message: EXPIRED_AUTH_TOKEN.to_string(),
                                        }
                                    )
                                )
                            } else {
                                Outcome::Failure(
                                    (
                                        Status::Unauthorized,
                                        StatusMessage {
                                            code: 401,
                                            status: Status::Unauthorized,
                                            message: AUTHENTICATION_FAILURE.to_string(),
                                        }
                                    )
                                )
                            }
                        }
                        None => {
                            Outcome::Failure(
                                (
                                    Status::Unauthorized,
                                    StatusMessage {
                                        code: 401,
                                        status: Status::Unauthorized,
                                        message: AUTHENTICATION_FAILURE.to_string(),
                                    }
                                )
                            )
                        }
                    }
                }
            }
        }
    }
}