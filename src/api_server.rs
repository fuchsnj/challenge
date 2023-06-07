use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

use crate::challenge::Challenge;
use crate::models::{SecretKeySubmission, SubmissionResponse};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppData {
    challenge: Arc<Mutex<Challenge>>,
}

async fn secret_handler(
    State(app_data): State<AppData>,
    Json(body): Json<SecretKeySubmission>,
) -> Json<SubmissionResponse> {
    let key_bytes = match base64::decode(&body.secret_key) {
        Ok(x) => x,
        Err(_) => {
            return Json(SubmissionResponse::failed(
                "Submission rejected: Invalid base64 encoding",
            ));
        }
    };

    match app_data
        .challenge
        .lock()
        .unwrap()
        .submit_secret_key(&key_bytes)
    {
        Ok(encrypted_secret) => Json(SubmissionResponse {
            secret_key_verified: true,
            encrypted_secret: Some(encrypted_secret),
            message: None,
        }),
        Err(_) => Json(SubmissionResponse::failed(
            "Invalid submission: wrong key or too late",
        )),
    }
}

pub async fn run(bind_addr: &'static str, challenge: Arc<Mutex<Challenge>>) {
    let app = Router::new()
        .route("/secret", post(secret_handler))
        .with_state(AppData { challenge });

    let addr = bind_addr.parse().expect("Invalid address");
    println!("API Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
