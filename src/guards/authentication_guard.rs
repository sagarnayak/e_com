use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;

use crate::jwt_master::jwt_master::validate_jwt;
use crate::model::status_message::StatusMessage;

pub struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
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
            // Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(key)=>{
                let validated_result = is_valid(key);
                if validated_result.0 {
                    Outcome::Success(ApiKey(key))
                }else  {
                    match validated_result.1 {
                        Some(is_expired)=>{
                            if is_expired {
                                Outcome::Failure(
                                    (
                                     4012,
                                        StatusMessage { code: 401, message: "You are not authorised".to_string() }
                                    )
                                )
                            }
                        }
                    }
                }
            } ,
            Some(_) => Outcome::Failure((Status::Unauthorized, StatusMessage { code: 401, message: "You are not authorised".to_string() })),
        }
    }
}