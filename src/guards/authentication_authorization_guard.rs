use std::collections::HashMap;

use base64::decode;
use jsonwebtoken::encode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use regex::Regex;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::config_controller::ConfigData;
use crate::contracts::blocked_for_platform_authorization_contracts::BlockedForPlatformAuthorizationContracts;
use crate::contracts::cached_auth_data_contracts::CachedAuthDataContracts;
use crate::core::constants::NEED_PLATFORM_AUTH;
use crate::core::strings::{AUTHENTICATION_FAILURE, AUTHORIZATION_FAILURE, EXPIRED_AUTH_TOKEN};
use crate::database::db_pool::DbPool;
use crate::jwt_master::jwt_master::{extract_jwt, validate_jwt};
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::blocked_for_platform_authorization::BlockedForPlatformAuthorization;
use crate::model::cached_auth_data::CachedAuthData;
use crate::model::claims::Claims;
use crate::model::google_jwt_response::GoogleJWTResponse;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;
use crate::model::user_test::{SignedAttestation, SignedAttestationResponse, UserTest};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationAuthorizationGuard {
    pub claims: Claims,
    pub allowed: bool,
    pub auth_expanded: Vec<AuthRolesCrossPaths>,
    pub jwt_hash: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticationAuthorizationGuard {
    type Error = StatusMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, StatusMessage> {
        fn is_valid(key: &str, config_data: ConfigData) -> (bool, Option<bool>, String) {
            let bearer_replaced = strip_bearer_string(&key);
            let validated_result = validate_jwt(&bearer_replaced, config_data.clone());
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

        async fn is_allowed<'r>(claims: &Claims, req: &'r Request<'_>) -> bool {
            let method: &Method = &req.method();
            let path = &req.uri().to_string();

            let db_pool = match req.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    println!("can not get db_pool in is_allowed()");
                    return false;
                }
            };

            let cached_auth_data = match CachedAuthData::get_data(
                &claims.auth_data_id,
                &db_pool,
            ).await {
                Ok(positive) => {
                    positive
                }
                Err(error) => {
                    println!("got error at getting cached auth data : {}", error.message);
                    return false;
                }
            };

            let auth_roles =
                cached_auth_data.auth_string.split(",").collect::<Vec<&str>>();

            for auth in auth_roles {
                let auth_expanded =
                    AuthRolesCrossPaths::full_version(auth.to_owned());
                if Regex::new(&format!(r"{}", auth_expanded.path)).unwrap().is_match(&path) {
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

        async fn should_perform_platform_authentication<'r>(
            user_id: &String,
            jwt: &String,
            request: &'r Request<'_>,
        )
            -> (bool, Option<String>) {
            // let should_block = rand::thread_rng().gen_bool(0.5);
            let should_block = false;

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
            jwt: &str,
            blocked_for_platform_authorization: &BlockedForPlatformAuthorization,
        ) -> bool {
            let config_data = match request.rocket().state::<ConfigData>() {
                Some(positive) => { positive }
                None => {
                    return false;
                }
            };

            let platform_auth_key = match request.headers().get_one("X-Platform-Authorization") {
                None => {
                    return false;
                }
                Some(positive) => {
                    positive
                }
            };

            let db_pool = match request.rocket().state::<DbPool>() {
                Some(positive) => { positive }
                None => {
                    return false;
                }
            };

            let client = reqwest::Client::new();

            let google_verification_url = format!(
                "https://www.googleapis.com/androidcheck/v1/attestations/verify?key={}",
                &config_data.google_api_key
            );
            println!("google url : {}", &google_verification_url);

            match client.post(
                google_verification_url
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
                            let jwt_data = platform_auth_key.split(".").collect::<Vec<&str>>()[1];
                            println!("The data to decode is : {}", jwt_data);
                            match decode(jwt_data) {
                                Ok(positive_inner_inner) => {
                                    println!("the decoded jwt is :: {:?}",
                                             std::str::from_utf8(&positive_inner_inner).unwrap()
                                    );
                                    let google_jwt_response: GoogleJWTResponse =
                                        serde_json::from_str(
                                            std::str::from_utf8(
                                                &positive_inner_inner).unwrap()
                                        ).unwrap();
                                    println!("the google jwt is :: {:?}", google_jwt_response.nonce);
                                    let encoded_nonce = base64::encode(
                                        &blocked_for_platform_authorization.nonce
                                    );
                                    if encoded_nonce != google_jwt_response.nonce {
                                        println!("failed to match nonce");
                                        return false;
                                    }
                                }
                                Err(error_inner_inner) => {
                                    println!("got error in decoding the jwt :: {}", error_inner_inner);
                                }
                            }
                            let jwt_to_resolve = jwt.split(".").collect::<Vec<&str>>()[2];
                            let if_done =
                                match BlockedForPlatformAuthorization::done_authorization_for_jwt_hash(
                                    jwt_to_resolve,
                                    db_pool,
                                ).await {
                                    Ok(positive_inner_inner) => {
                                        positive_inner_inner
                                    }
                                    Err(error_inner) => {
                                        println!("error is inserting done auth data to DB : {:?}", &error_inner);
                                        return false;
                                    }
                                };
                            return if_done;
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

        async fn got_jwt_key<'r>(jwt_key: &str, req: &'r Request<'_>)
                                 -> Outcome<AuthenticationAuthorizationGuard, StatusMessage> {
            let config_data = match req.rocket().state::<ConfigData>() {
                Some(positive) => { positive }
                None => {
                    println!("can not get donfig data in got_jwt_key()");
                    return Outcome::Failure(
                        (
                            Status::BadRequest,
                            StatusMessage {
                                code: Status::BadRequest.code,
                                status: Status::BadRequest,
                                message: "Failed to get config_data data".to_owned(),
                                sys_message: None,
                            }
                        )
                    );
                }
            };
            let validated_result = is_valid(jwt_key, config_data.clone());
            if validated_result.0 {
                match extract_jwt(&validated_result.2, config_data.clone()) {
                    Ok(claims) => {
                        let db_pool = match req.rocket().state::<DbPool>() {
                            Some(positive) => { positive }
                            None => {
                                println!("can not get db_pool in got_jwt_key()");
                                return Outcome::Failure(
                                    (
                                        Status::BadRequest,
                                        StatusMessage {
                                            code: Status::BadRequest.code,
                                            status: Status::BadRequest,
                                            message: "Failed to get db_pool data".to_owned(),
                                            sys_message: None,
                                        }
                                    )
                                );
                            }
                        };

                        let cached_auth_data = match CachedAuthData::get_data(
                            &claims.auth_data_id,
                            &db_pool,
                        ).await {
                            Ok(positive) => {
                                positive
                            }
                            Err(error) => {
                                println!("got error at getting cached auth data : {}", error.message);
                                return Outcome::Failure(
                                    (
                                        Status::BadRequest,
                                        StatusMessage {
                                            code: Status::BadRequest.code,
                                            status: Status::BadRequest,
                                            message: "Failed to get db_pool data".to_owned(),
                                            sys_message: None,
                                        }
                                    )
                                );
                            }
                        };

                        let auth_roles =
                            cached_auth_data.auth_string.split(",").collect::<Vec<&str>>();

                        let mut auth_roles_expanded: Vec<AuthRolesCrossPaths> = vec![];

                        for auth in auth_roles {
                            let auth_expanded =
                                AuthRolesCrossPaths::full_version(auth.to_owned());
                            auth_roles_expanded.push(auth_expanded);
                        }

                        if is_allowed(
                            &claims,
                            req,
                        ).await {
                            let jwt_hash = jwt_key.split(".").collect::<Vec<&str>>()[2];
                            let striped_jwt = &strip_bearer_string(&jwt_key);
                            let if_present_in_blocked_for_platform_authorization_list_result =
                                if_present_in_blocked_for_platform_authorization_list(
                                    &striped_jwt,
                                    &req,
                                ).await;
                            if if_present_in_blocked_for_platform_authorization_list_result.0 {
                                let verify_platform_authorization_result =
                                    verify_platform_authorization(
                                        &req,
                                        &jwt_key,
                                        &if_present_in_blocked_for_platform_authorization_list_result.1.unwrap(),
                                    ).await;
                                if verify_platform_authorization_result {
                                    return Outcome::Success(
                                        AuthenticationAuthorizationGuard {
                                            claims,
                                            allowed: true,
                                            auth_expanded: auth_roles_expanded,
                                            jwt_hash: jwt_hash.to_owned(),
                                        }
                                    );
                                }
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
                                    auth_expanded: auth_roles_expanded,
                                    jwt_hash: jwt_hash.to_owned(),
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
                            let mut paths_to_skip_for_validity_test: Vec<String> = vec![];
                            paths_to_skip_for_validity_test.push("/renewToken".to_owned());
                            let path = &req.uri().to_string();
                            if paths_to_skip_for_validity_test.contains(path) {
                                let data_in_jwt = &validated_result.2.split(".").collect::<Vec<&str>>()[1];
                                let claims = match decode(data_in_jwt) {
                                    Ok(positive_inner_inner) => {
                                        let claims: Claims =
                                            serde_json::from_str(
                                                std::str::from_utf8(
                                                    &positive_inner_inner).unwrap()
                                            ).unwrap();

                                        claims
                                    }
                                    Err(_) => {
                                        return Outcome::Failure(
                                            (
                                                Status::Unauthorized,
                                                StatusMessage {
                                                    code: 401,
                                                    status: Status::Unauthorized,
                                                    message: EXPIRED_AUTH_TOKEN.to_string(),
                                                    sys_message: None,
                                                }
                                            )
                                        );
                                    }
                                };
                                let db_pool = match req.rocket().state::<DbPool>() {
                                    Some(positive) => { positive }
                                    None => {
                                        println!("can not get db_pool in got_jwt_key()");
                                        return Outcome::Failure(
                                            (
                                                Status::BadRequest,
                                                StatusMessage {
                                                    code: Status::BadRequest.code,
                                                    status: Status::BadRequest,
                                                    message: "Failed to get db_pool data".to_owned(),
                                                    sys_message: None,
                                                }
                                            )
                                        );
                                    }
                                };
                                let cached_auth_data = match CachedAuthData::get_data(
                                    &claims.auth_data_id,
                                    &db_pool,
                                ).await {
                                    Ok(positive) => {
                                        positive
                                    }
                                    Err(error) => {
                                        println!("got error at getting cached auth data : {}", error.message);
                                        return Outcome::Failure(
                                            (
                                                Status::BadRequest,
                                                StatusMessage {
                                                    code: Status::BadRequest.code,
                                                    status: Status::BadRequest,
                                                    message: "Failed to get db_pool data".to_owned(),
                                                    sys_message: None,
                                                }
                                            )
                                        );
                                    }
                                };

                                let auth_roles =
                                    cached_auth_data.auth_string.split(",").collect::<Vec<&str>>();

                                let mut auth_roles_expanded: Vec<AuthRolesCrossPaths> = vec![];

                                for auth in auth_roles {
                                    let auth_expanded =
                                        AuthRolesCrossPaths::full_version(auth.to_owned());
                                    auth_roles_expanded.push(auth_expanded);
                                }
                                let jwt_hash = jwt_key.split(".").collect::<Vec<&str>>()[2];
                                return Outcome::Success(
                                    AuthenticationAuthorizationGuard {
                                        claims,
                                        allowed: true,
                                        auth_expanded: auth_roles_expanded,
                                        jwt_hash: jwt_hash.to_owned(),
                                    }
                                );
                            }
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
                            println!("1");
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
                        println!("2");
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

        match req.headers().get_one("Authorization") {
            None => {
                println!("3");
                Outcome::Failure((Status::Unauthorized, StatusMessage {
                    code: 401,
                    status: Status::Unauthorized,
                    message: AUTHENTICATION_FAILURE.to_string(),
                    sys_message: None,
                }
                )
                )
            }
            Some(key) => {
                return got_jwt_key(&key, &req).await;
            }
        }
    }
}