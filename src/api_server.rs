use tower_web::ServiceBuilder;
use tokio::prelude::*;
use std::sync::{Arc, Mutex};
use crate::challenge::Challenge;
use crate::models::{SecretKeySubmission, SubmissionResponse};
use std::thread::JoinHandle;
use std::thread;

#[derive(Clone)]
pub struct ApiServer {
    challenge: Arc<Mutex<Challenge>>,
}

/// impl_web! is a temporary macro to enable tower-web to work with Rust stable.
/// In the next major version release, this will be transitioned to use attribute macros.
/// see https://github.com/carllerche/tower-web/issues/194
impl_web! {
    impl ApiServer {

        #[post("/secret")]
        #[content_type("application/json")]
        fn check_secret_submission(&self, body: SecretKeySubmission) -> Result<SubmissionResponse, ()> {
            let key_bytes = match base64::decode(&body.secret_key){
                Ok(x) => x,
                Err(_) => {return Ok(SubmissionResponse::failed("Submission rejected: Invalid base64 encoding"));}
            };


            match self.challenge.lock().unwrap().submit_secret_key(&key_bytes) {
                Ok(encrypted_secret) => {
                    Ok(SubmissionResponse{
                        secret_key_verified: true,
                        encrypted_secret: Some(encrypted_secret),
                        message: None
                    })
                },
                Err(_) => {
                    Ok(SubmissionResponse::failed("Invalid submission: wrong key or too late"))
                }
            }
        }
    }
}

pub fn run(bind_addr: &'static str, challenge: Arc<Mutex<Challenge>>) {
    let addr = bind_addr.parse().expect("Invalid address");
    println!("API Server listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ApiServer { challenge })
        .run(&addr)
        .unwrap();
}

pub fn start(bind_addr: &'static str, challenge: Arc<Mutex<Challenge>>) -> JoinHandle<()> {
    thread::spawn(move || {
        run(bind_addr, challenge);
    })
}