use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub jwt: String,
    pub refresh_token: String,
}