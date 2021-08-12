use rand::Rng;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
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
            let path = if path.contains("?") {
                path.split("?").collect::<Vec<&str>>()[0]
            } else {
                path
            };
            let path = if path.contains("/") {
                let striped_path = path.split("/").collect::<Vec<&str>>()[1];
                format!("/{}", &striped_path)
            } else {
                path.to_owned()
            };

            for auth in claims.authorizations_minified.clone() {
                let auth_expanded = AuthRolesCrossPaths::full_version(auth);
                if &auth_expanded.path == &path {
                    match method {
                        Method::Get => {
                            if auth_expanded.get_allowed {
                                return true;
                            }
                        }
                        Method::Post => {
                            if auth_expanded.post_allowed {
                                return true;
                            }
                        }
                        Method::Put => {
                            if auth_expanded.put_allowed {
                                return true;
                            }
                        }
                        Method::Delete => {
                            if auth_expanded.delete_allowed {
                                return true;
                            }
                        }
                        _ => return false
                    }
                }
            }

            return false;
        }

        async fn should_perform_platform_authentication<'r>(jwt: &String, request: &'r Request<'_>) -> bool {
            let should_block = rand::thread_rng().gen_bool(0.5);

            if !should_block {
                return should_block;
            }

            let db_pool = match request.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    return false;
                }
            };

            match BlockedForPlatformAuthorization::add_jwt(jwt, &db_pool).await {
                Ok(_) => {
                    return true;
                }
                Err(_) => {
                    return false;
                }
            }
        }

        async fn if_present_in_blocked_for_platform_authorization_list<'r>(jwt: &String, request: &'r Request<'_>) -> bool {
            let db_pool = match request.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    return false;
                }
            };

            return match BlockedForPlatformAuthorization::find_data(
                &jwt,
                &db_pool,
            ).await {
                Ok(_) => {
                    true
                }
                Err(_) => {
                    false
                }
            };
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
                            if is_allowed(
                                &claims,
                                req,
                            ) {
                                let striped_jwt = &strip_bearer_string(key);
                                if if_present_in_blocked_for_platform_authorization_list(
                                    &striped_jwt,
                                    &req,
                                ).await || should_perform_platform_authentication(
                                    &striped_jwt,
                                    &req,
                                ).await {
                                    return Outcome::Failure(
                                        (
                                            Status::Unauthorized,
                                            StatusMessage {
                                                code: NEED_PLATFORM_AUTH,
                                                status: Status::Unauthorized,
                                                message: "Please perform a platform authorization".to_owned(),
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