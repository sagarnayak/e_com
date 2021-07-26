use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordVerificationRequest {
    pub original_password: String,
    pub hash: String,
}