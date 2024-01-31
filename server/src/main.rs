mod handle_client;

use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;

fn bind_server() -> std::io::Result<TcpListener> {
    match TcpListener::bind("127.0.0.1:4242") {
        Ok(listener) => {
            println!("Listening on {}", listener.local_addr().unwrap());
            Ok(listener)
        }
        Err(e) => {
            println!("Unable to bind: {e}");
            Err(e)
        }
    }
}

fn main() {
    let listener = match bind_server() {
        Ok(listener) => listener,
        Err(_) => exit(1),
    };
    let mut id: u32 = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                id += 1;
                thread::spawn(move || handle_client::Client::new_client(id, stream));
            }
            Err(e) => println!("Error: {e}"),
        }
    }
}
