mod api_server;
mod challenge;
mod crypto;
mod models;
mod tcp_server;

use crate::challenge::Challenge;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIME_BETWEEN_ROUNDS: Duration = Duration::from_secs(30);

const TCP_SERVER_BIND_ADDR: &'static str = "0.0.0.0:8162";
const API_SERVER_BIND_ADDR: &'static str = "0.0.0.0:8080";

fn get_secret_string() -> String {
    print!("Enter secret string: ");
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    output.trim().to_owned() // remove newline char
}

#[tokio::main]
async fn main() {
    let secret_string = get_secret_string();
    println!();

    let challenge = Arc::new(Mutex::new(Challenge::new(&secret_string)));

    {
        let challenge = challenge.clone();
        std::thread::spawn(move || {
            tcp_server::run(TCP_SERVER_BIND_ADDR, challenge);
        });
    }

    tokio::spawn(api_server::run(API_SERVER_BIND_ADDR, challenge.clone()));

    loop {
        thread::sleep(TIME_BETWEEN_ROUNDS);
        challenge.lock().unwrap().start_new_round();
    }
}
