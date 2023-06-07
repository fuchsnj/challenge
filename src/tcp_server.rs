use crate::challenge::Challenge;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

fn handle_client(stream: TcpStream, challenge: &mut Challenge) {
    // We have a mutable reference to Challenge here, so Rust guarantees nothing else
    // can access the same instance of Challenge while this mutable reference is valid
    challenge.add_player(stream);
}

pub fn run(bind_addr: &str, challenge: Arc<Mutex<Challenge>>) {
    println!("TCP Server listening on {}", bind_addr);
    for stream_result in TcpListener::bind(bind_addr).unwrap().incoming() {
        if let Ok(stream) = stream_result {
            // Locking the mutex returns a `MutexGuard<T>` here, which is a smart pointer around T.

            // We take a mutable reference to the MutexGuard, but since MutexGuard implements Deref
            // it's able to be treated as a reference to T since that's what the function we are calling expects

            // Also note that it's not possible to access the data before a lock is obtained, and the lock
            // cannot be released while the data is still being used
            handle_client(stream, &mut challenge.lock().unwrap());
        }
    }
}
