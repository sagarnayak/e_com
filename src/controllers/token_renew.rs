use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::refresh_token_contracts::RefreshTokenContracts;
use crate::contracts::user_contracts::UserContracts;
use crate::database::db_pool::DbPool;
use crate::guards::authentication_authorization_guard::AuthenticationAuthorizationGuard;
use crate::jwt_master::jwt_master::{create_jwt, extract_refresh_jwt, validate_refresh_jwt};
use crate::model::authentication_response::AuthenticationResponse;
use crate::model::refresh_token_log::{RefreshTokenLog, RefreshTokenUsedReason};
use crate::model::status_message::StatusMessage;
use crate::model::token_renew_request::TokenRenewRequest;
use crate::model::user::User;

#[post("/renewToken", data = "<token_renew_request>")]
pub async fn renew_token(
    authentication_authorization_guard: Result<AuthenticationAuthorizationGuard, StatusMessage>,
    db_pool: &State<DbPool>,
    token_renew_request: Option<Json<TokenRenewRequest>>,
) -> status::Custom<Result<Json<AuthenticationResponse>, Json<StatusMessage>>> {
    let authentication_authorization_guard =
        match authentication_authorization_guard {
            Ok(positive) => { positive }
            Err(error) => {
                return StatusMessage::dynamic_error_with_status_code_in_result(error);
            }
        };

    let token_renew_request = match token_renew_request {
        Some(positive) => {
            positive
        }
        None => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                "Please provide refresh token".to_owned()
            );
        }
    };

    let refresh_validate_results = validate_refresh_jwt(&token_renew_request.refresh_token);

    if !refresh_validate_results.0 {
        return StatusMessage::bad_request_400_with_status_code_in_result(
            "Invalid refresh token".to_owned()
        );
    }

    if refresh_validate_results.1 {
        return StatusMessage::bad_request_400_with_status_code_in_result(
            "Refresh token has expired.".to_owned()
        );
    }

    let refresh_token_claims = match extract_refresh_jwt(&token_renew_request.refresh_token) {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    if refresh_token_claims.jwt_hash != authentication_authorization_guard.jwt_hash {
        RefreshTokenLog::refresh_token_used(
            &refresh_token_claims.jwt_hash,
            RefreshTokenUsedReason::TryingToForge(
                format!(
                    "User {} is trying to use token which does not belong to it",
                    &authentication_authorization_guard.claims.owner
                )
            ),
            &db_pool,
        ).await;
        return StatusMessage::bad_request_400_with_status_code_in_result(
            "You are using refresh token which does not belong to you.".to_owned()
        );
    }

    let check_if_used_result = match RefreshTokenLog::check_if_used(
        &refresh_token_claims.jwt_hash,
        &db_pool,
    ).await {
        Ok(positive) => { positive }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(error.message);
        }
    };

    if check_if_used_result {
        return StatusMessage::bad_request_400_with_status_code_in_result(
            "Refresh token is already used.".to_owned()
        );
    }

    let user: User = match User::find_user_with_id(
        authentication_authorization_guard.claims.owner.to_owned(),
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_with_status_code_in_result(
                error.message
            );
        }
    };

    let created_jwt_and_refresh_pair = match create_jwt(
        60 * 60,
        60 * 60 * 24 * 30,
        &user,
        authentication_authorization_guard.auth_expanded,
        &db_pool,
    ).await {
        Ok(positive) => positive,
        Err(error) => {
            return status::Custom(
                Status::BadRequest,
                Err(Json(error)),
            );
        }
    };

    RefreshTokenLog::refresh_token_used(
        &refresh_token_claims.jwt_hash,
        RefreshTokenUsedReason::NormalUse(
            format!("Used by valid user {}", &authentication_authorization_guard.claims.owner)
        ),
        &db_pool,
    ).await;

    status::Custom(
        Status::Ok,
        Ok(
            Json(
                AuthenticationResponse {
                    jwt: created_jwt_and_refresh_pair.0,
                    refresh_token: created_jwt_and_refresh_pair.1,
                }
            )
        ),
    )
}