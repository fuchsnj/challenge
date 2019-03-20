#[macro_use]
extern crate tower_web;

mod crypto;
mod challenge;
mod tcp_server;
mod api_server;
mod models;

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io;
use crate::challenge::Challenge;
use std::io::{Write, Read};
use std::time::Duration;

const TIME_BETWEEN_ROUNDS: Duration = Duration::from_secs(30);

const TCP_SERVER_BIND_ADDR: &'static str = "0.0.0.0:8162";
const API_SERVER_BIND_ADDR: &'static str = "0.0.0.0:8080";

fn get_secret_string() -> String {
    print!("Enter secret string: ");
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output);
    output.trim().to_owned() // remove newline char
}

fn main() {
    let secret_string = get_secret_string();
    let challenge = Arc::new(Mutex::new(Challenge::new(&secret_string)));

    tcp_server::start(TCP_SERVER_BIND_ADDR, challenge.clone());
    api_server::start(API_SERVER_BIND_ADDR, challenge.clone());

    loop {
        thread::sleep(TIME_BETWEEN_ROUNDS);
        challenge.lock().unwrap().start_new_round();
    }
}