use crate::challenge::Challenge;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{io, thread};

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

            // The MutexGuard goes out of scope here, and is automatically unlocked because MutexGuard
            // implements the Drop trait, and releases the lock in the Drop implementation
        }
    }
}

// pub fn start(bind_addr: &'static str, challenge: Arc<Mutex<Challenge>>) -> JoinHandle<()> {
//     thread::spawn(move || {
//         // Data captured by a closure being passed to a thread must live for the 'static lifetime, meaning
//         // it needs to be valid for the life of the entire program. This is accomplished 2 different ways.

//         // The bind_addr is a reference to a string, so we just add a 'static lifetime constraint to the
//         // reference and Rust guarantees you can only pass a parameter that will live for the 'static lifetime.

//         // The challenge parameter is not a reference, so it must take ownership of the data by `move`ing the ownership
//         // to the thread's closure. This is accomplished by adding the keyword `move` to the front of the closure.

//         run(bind_addr, challenge);
//     })
// }
