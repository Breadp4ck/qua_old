use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

impl AuthPayload {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

pub struct AuthService;

impl AuthService {
    pub async fn authorize(payload: AuthPayload) -> AuthBody {
        reqwest::Client::new()
            .post("http://localhost:8000/api/auth/authorize")
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
            .json::<AuthBody>()
            .await
            .expect("Failed authorize.")
    }
}
