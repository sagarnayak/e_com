use std::collections::HashMap;

use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::contracts::blocked_for_platform_authorization_contracts::BlockedForPlatformAuthorizationContracts;
use crate::core::constants::NEED_PLATFORM_AUTH;
use crate::core::strings::{AUTHENTICATION_FAILURE, AUTHORIZATION_FAILURE, EXPIRED_AUTH_TOKEN};
use crate::database::db_pool::DbPool;
use crate::jwt_master::jwt_master::{extract_jwt, validate_jwt};
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::claims::Claims;
use crate::model::status_message::StatusMessage;
use crate::model::user_test::{SignedAttestation, UserTest, SignedAttestationResponse};
use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationAuthorizationGuard {
    pub claims: Claims,
    pub allowed: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticationAuthorizationGuard {
    type Error = StatusMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, StatusMessage> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> (bool, Option<bool>, String) {
            let bearer_replaced = strip_bearer_string(&key);

            let validated_result = validate_jwt(&bearer_replaced);

            (validated_result.0, Some(validated_result.1), bearer_replaced.to_owned())
        }

        fn strip_bearer_string(key: &str) -> String {
            let if_bearer_token_found: bool = key.contains("Bearer ");
            if !if_bearer_token_found {
                return key.to_owned();
            }

            let bearer_replaced = key.replace("Bearer ", "");

            return bearer_replaced;
        }

        fn is_allowed(claims: &Claims, req: &Request) -> bool {
            let method: &Method = &req.method();
            let path = &req.uri().to_string();
            println!("path to test :: {}", &path);
            let path = if path.contains("?") {
                path.split("?").collect::<Vec<&str>>()[0]
            } else {
                path
            };
            println!("removed ? :: {}", &path);
            let path = if path.contains("/") {
                let striped_path = path.split("/").collect::<Vec<&str>>()[1];
                format!("/{}", &striped_path)
            } else {
                path.to_owned()
            };
            println!("removed / :: {}", &path);

            for auth in claims.authorizations_minified.clone() {
                let auth_expanded = AuthRolesCrossPaths::full_version(auth);
                if &auth_expanded.path == &path {
                    println!("matched path :: {}", &path);
                    match method {
                        Method::Get => {
                            if auth_expanded.get_allowed {
                                println!("matched get");
                                return true;
                            }
                        }
                        Method::Post => {
                            if auth_expanded.post_allowed {
                                println!("matched post");
                                return true;
                            }
                        }
                        Method::Put => {
                            if auth_expanded.put_allowed {
                                println!("matched put");
                                return true;
                            }
                        }
                        Method::Delete => {
                            if auth_expanded.delete_allowed {
                                println!("matched delete");
                                return true;
                            }
                        }
                        _ => return false
                    }
                }
            }

            return false;
        }

        async fn should_perform_platform_authentication<'r>(user_id: &String, jwt: &String, request: &'r Request<'_>) -> (bool, Option<String>) {
            let should_block = rand::thread_rng().gen_bool(0.5);

            if !should_block {
                return (false, None);
            }

            let db_pool = match request.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    println!("blocked due to db pool not found");
                    return (false, None);
                }
            };

            let rand_string: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();

            return match BlockedForPlatformAuthorization::add_jwt(
                &user_id,
                &jwt,
                &rand_string,
                &db_pool,
            ).await {
                Ok(_) => {
                    println!("blocked after inserting the data to the table");
                    (true, Some(rand_string))
                }
                Err(_) => {
                    println!("not blocked due to inserting failure");
                    (false, None)
                }
            };
        }

        async fn if_present_in_blocked_for_platform_authorization_list<'r>(
            jwt: &String,
            request: &'r Request<'_>,
        ) -> (bool, Option<BlockedForPlatformAuthorization>) {
            let db_pool = match request.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    return (false, None);
                }
            };

            return match BlockedForPlatformAuthorization::find_data_with_jwt(
                &jwt,
                &db_pool,
            ).await {
                Ok(positive) => {
                    println!("found jwt in blocked for platform check");
                    (true, Some(positive))
                }
                Err(_) => {
                    println!("not found jwt in blocked for platform check");
                    (false, None)
                }
            };
        }

        async fn verify_platform_authorization<'r>(
            request: &'r Request<'_>,
        ) -> bool {
            let platform_auth_key = match request.headers().get_one("X-Platform-Authorization") {
                None => {
                    return false;
                }
                Some(positive) => {
                    positive
                }
            };

            let client = reqwest::Client::new();

            match client.post(
                "https://www.googleapis.com/androidcheck/v1/attestations/verify?key=AIzaSyCmJeN7rfeIYtuL-J-_PMlP8dvTkNL2NEg"
            )
                .json(
                    &SignedAttestation {
                        signedAttestation: platform_auth_key.to_owned()
                    }
                )
                .send()
                .await {
                Ok(positive) => {
                    println!("Response from google :: {:?}", &positive);
                    return match positive.json::<SignedAttestationResponse>().await {
                        Ok(positive_inner) => {
                            println!("inner positive :: {:?}", &positive_inner);
                            false
                        }
                        Err(error_inner) => {
                            println!("inner error is :: {}", error_inner.to_string());
                            false
                        }
                    };
                }
                Err(error) => {
                    println!("error is :: {}", error.to_string());
                    return false;
                }
            };
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, StatusMessage {
                code: 401,
                status: Status::Unauthorized,
                message: AUTHENTICATION_FAILURE.to_string(),
                sys_message: None,
            })),
            Some(key) => {
                let validated_result = is_valid(key);
                if validated_result.0 {
                    match extract_jwt(&validated_result.2) {
                        Ok(claims) => {
                            if is_allowed(
                                &claims,
                                req,
                            ) {
                                let verify_platform_authorization_result = verify_platform_authorization(
                                    &req
                                ).await;
                                let striped_jwt = &strip_bearer_string(key);
                                let if_present_in_blocked_for_platform_authorization_list_result =
                                    if_present_in_blocked_for_platform_authorization_list(
                                        &striped_jwt,
                                        &req,
                                    ).await;
                                if if_present_in_blocked_for_platform_authorization_list_result.0 {
                                    let verify_platform_authorization_result = verify_platform_authorization(
                                        &req
                                    ).await;
                                    return Outcome::Failure(
                                        (
                                            Status::Unauthorized,
                                            StatusMessage {
                                                code: NEED_PLATFORM_AUTH,
                                                status: Status::Unauthorized,
                                                message: "Please perform a platform authorization.".to_owned(),
                                                sys_message: None,
                                            }
                                        )
                                    );
                                }
                                let should_perform_platform_authentication_result =
                                    should_perform_platform_authentication(
                                        &claims.owner,
                                        &striped_jwt,
                                        &req,
                                    ).await;
                                if should_perform_platform_authentication_result.0 {
                                    return Outcome::Failure(
                                        (
                                            Status::Unauthorized,
                                            StatusMessage {
                                                code: NEED_PLATFORM_AUTH,
                                                status: Status::Unauthorized,
                                                message: "Please perform a platform authorization".to_owned(),
                                                sys_message: Some(should_perform_platform_authentication_result.1.unwrap()),
                                            }
                                        )
                                    );
                                }
                                Outcome::Success(
                                    AuthenticationAuthorizationGuard {
                                        claims,
                                        allowed: true,
                                    }
                                )
                            } else {
                                Outcome::Failure(
                                    (
                                        Status::Forbidden,
                                        StatusMessage {
                                            code: 403,
                                            status: Status::Forbidden,
                                            message: AUTHORIZATION_FAILURE.to_string(),
                                            sys_message: None,
                                        }
                                    )
                                )
                            }
                        }
                        Err(status_message) => {
                            Outcome::Failure(
                                (
                                    Status::Unauthorized,
                                    StatusMessage {
                                        code: 401,
                                        status: Status::Unauthorized,
                                        message: status_message.message,
                                        sys_message: None,
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
                                            sys_message: None,
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
                                            sys_message: None,
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
                                        sys_message: None,
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