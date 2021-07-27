use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::jwt_master::jwt_master::validate_jwt;
use crate::model::status_message::StatusMessage;
use rocket::http::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationGuard<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticationGuard<'r> {
    type Error = StatusMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, StatusMessage> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> (bool, Option<bool>) {
            let if_bearer_token_found: bool = key.contains("Bearer ");
            if !if_bearer_token_found {
                return (false, None);
            }

            let bearer_replaced = key.replace("Bearer ", "");

            let validated_result = validate_jwt(bearer_replaced);

            (validated_result.0, Some(validated_result.1))
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, StatusMessage { code: 401, message: "You are not authorised".to_string() })),
            Some(key) => {
                let validated_result = is_valid(key);
                if validated_result.0 {
                    Outcome::Success(AuthenticationGuard(key))
                } else {
                    match validated_result.1 {
                        Some(is_expired) => {
                            if is_expired {
                                Outcome::Failure(
                                    (
                                        Status::Unauthorized,
                                        StatusMessage { code: 401, message: "You are not authorised".to_string() }
                                    )
                                )
                            } else {
                                Outcome::Failure(
                                    (
                                        Status::Unauthorized,
                                        StatusMessage { code: 401, message: "You are not authorised".to_string() }
                                    )
                                )
                            }
                        }
                        None => {
                            Outcome::Failure(
                                (
                                    Status::Unauthorized,
                                    StatusMessage { code: 401, message: "You are not authorised".to_string() }
                                )
                            )
                        }
                    }
                }
            }
        }
    }
}