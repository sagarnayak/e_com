use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::serde::json::serde_json::ser::CharEscape::CarriageReturn;

use crate::config_controller::ConfigData;
use crate::core::strings::FAILED_TO_CREATE_JWT;
use crate::model::claims::Claims;
use crate::model::status_message::StatusMessage;

pub fn create_jwt(
    exp_after_secs: i64,
    owner: String,
) -> Result<String, StatusMessage> {
    let my_claims =
        Claims {
            owner,
            exp: (Utc::now().timestamp() + exp_after_secs) as usize,
        };

    let mut header = Header::default();
    header.kid = Some("key identifier".to_string());
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