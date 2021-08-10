use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;

use crate::config_controller::ConfigData;
use crate::core::strings::FAILED_TO_CREATE_JWT;
use crate::model::auth_roles_cross_paths::AuthRolesCrossPaths;
use crate::model::claims::Claims;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

pub fn create_jwt(
    exp_after_secs: i64,
    user: &User,
    auth_roles_cross_paths: Vec<AuthRolesCrossPaths>,
) -> Result<String, StatusMessage> {
    let mut minified_auth_roles_cross_paths = vec![];
    for auth in auth_roles_cross_paths {
        minified_auth_roles_cross_paths.push(
            auth.get_minified_version()
        )
    }
    let my_claims =
        Claims {
            owner: user.id.clone(),
            authorizations_minified: minified_auth_roles_cross_paths,
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

    Ok(token)
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