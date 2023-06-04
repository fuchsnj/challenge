use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

// use tower_web::ServiceBuilder;
use crate::challenge::Challenge;
use crate::models::{SecretKeySubmission, SubmissionResponse};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone)]
pub struct AppData {
    challenge: Arc<Mutex<Challenge>>,
}

// /// impl_web! is a temporary macro to enable tower-web to work with Rust stable.
// /// In the next major version release, this will be transitioned to use attribute macros.
// /// see https://github.com/carllerche/tower-web/issues/194
// impl_web! {
//     impl ApiServer {

//         #[post("/secret")]
//         #[content_type("application/json")]
//         fn check_secret_submission(&self, body: SecretKeySubmission) -> Result<SubmissionResponse, ()> {
//             let key_bytes = match base64::decode(&body.secret_key){
//                 Ok(x) => x,
//                 Err(_) => {return Ok(SubmissionResponse::failed("Submission rejected: Invalid base64 encoding"));}
//             };

//             match self.challenge.lock().unwrap().submit_secret_key(&key_bytes) {
//                 Ok(encrypted_secret) => {
//                     Ok(SubmissionResponse{
//                         secret_key_verified: true,
//                         encrypted_secret: Some(encrypted_secret),
//                         message: None
//                     })
//                 },
//                 Err(_) => {
//                     Ok(SubmissionResponse::failed("Invalid submission: wrong key or too late"))
//                 }
//             }
//         }
//     }
// }

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

// pub fn start(bind_addr: &'static str, challenge: Arc<Mutex<Challenge>>) -> JoinHandle<()> {
//     thread::spawn(move || {
//         run(bind_addr, challenge);
//     })
// }
