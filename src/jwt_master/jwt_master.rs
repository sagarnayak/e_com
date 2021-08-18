use std::ops::Add;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;

use crate::config_controller::ConfigData;
use crate::contracts::cached_auth_data_contracts::CachedAuthDataContracts;
use crate::core::strings::FAILED_TO_CREATE_JWT;
use crate::database::db_pool::DbPool;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::cached_auth_data::CachedAuthData;
use crate::model::claims::Claims;
use crate::model::refresh_claims::RefreshClaims;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

pub async fn create_jwt(
    exp_after_secs: i64,
    exp_after_secs_for_refresh: i64,
    user: &User,
    auth_roles_cross_paths: Vec<AuthRolesCrossPaths>,
    db_pool: &DbPool,
) -> Result<(String, String), StatusMessage> {
    let mut minified_auth_roles_cross_paths: String = "".to_owned();
    for auth in auth_roles_cross_paths {
        let minified_version = auth.get_minified_version();
        if minified_auth_roles_cross_paths.len() == 0 {
            minified_auth_roles_cross_paths = minified_version;
        } else {
            minified_auth_roles_cross_paths.push_str(
                &format!(
                    ",{}",
                    &minified_version
                )
            )
        }
    }
    let cached_data_id = match CachedAuthData::insert_new(
        &minified_auth_roles_cross_paths,
        Utc::now() + Duration::seconds(exp_after_secs + 2),
        &db_pool,
    ).await {
        Ok(positive) => {
            positive
        }
        Err(error) => {
            return StatusMessage::bad_request_400_in_result_with_status_message(error);
        }
    };

    let my_claims =
        Claims {
            owner: user.id.clone(),
            auth_data_id: cached_data_id,
            exp: (Utc::now().timestamp() + exp_after_secs) as usize,
        };

    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    let token = match encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(ConfigData::new().jwt.secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => return StatusMessage::bad_request_400_in_result(FAILED_TO_CREATE_JWT.to_string()),
    };

    let refresh_claims = RefreshClaims {
        owner: user.id.clone(),
        jwt_hash: token.split(".").collect::<Vec<&str>>()[2].to_owned(),
        exp: (Utc::now().timestamp() + (exp_after_secs_for_refresh)) as usize,
    };

    let refresh_token = match encode(
        &header,
        &refresh_claims,
        &EncodingKey::from_secret(ConfigData::new().jwt.secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => return StatusMessage::bad_request_400_in_result(FAILED_TO_CREATE_JWT.to_string()),
    };

    Ok((token, refresh_token))
}

pub fn validate_jwt(jwt: &str) -> (bool, bool) {
    let mut is_valid = false;
    let mut is_expired = false;

    let _ = match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(
            ConfigData::new().jwt.secret.as_bytes()
        ),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(_) => {
            is_valid = true;
        }
        Err(err) => {
            match *err.kind() {
                ErrorKind::InvalidToken => {}
                ErrorKind::InvalidIssuer => {}
                ErrorKind::ExpiredSignature => {
                    is_expired = true;
                }
                _ => {}
            };
        }
    };

    return (is_valid, is_expired);
}

pub fn extract_jwt(key_to_decode: &str) -> Result<Claims, StatusMessage> {
    let token_data = match decode::<Claims>(
        &key_to_decode,
        &DecodingKey::from_secret(
            ConfigData::new().jwt.secret.as_bytes()
        ),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => Ok(c.claims),
        Err(_) => {
            StatusMessage::bad_request_400_in_result("Failed to extract data from JWT".to_string())
        }
    };

    token_data
}