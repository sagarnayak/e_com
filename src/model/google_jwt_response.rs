use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleJWTResponse {
    pub nonce: String,
    pub timestamp_ms: usize,
    pub apk_package_name: String,
    pub apk_digest_sha256: String,
    pub cts_profile_match: bool,
    pub apk_certificate_digest_sha256: Vec<String>,
    pub basic_integrity: bool,
    pub evaluation_type: String,
}