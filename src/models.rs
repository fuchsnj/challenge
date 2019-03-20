use serde::{Serialize, Deserialize};

#[derive(Extract, Debug, Serialize, Deserialize)]
pub struct SecretKeySubmission {
    pub secret_key: String
}

#[derive(Debug, Serialize, Deserialize, Response)]
pub struct SubmissionResponse {
    pub secret_key_verified: bool,
    pub encrypted_secret: Option<String>,
    // base64-encoded encrypted secret
    pub message: Option<String>,
}

impl SubmissionResponse {
    pub fn failed(msg: &str) -> SubmissionResponse {
        SubmissionResponse {
            secret_key_verified: false,
            encrypted_secret: None,
            message: Some(msg.to_owned()),
        }
    }
}