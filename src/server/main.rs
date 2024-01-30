mod handle_client;

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::process::exit;

fn bind_server() -> std::io::Result<TcpListener> {
    match TcpListener::bind("127.0.0.1:4242") {
        Ok(listener) => {
            println!("Listening on {}", listener.local_addr().unwrap());
            Ok(listener)
        },
        Err(e) => {
            println!("Unable to bind: {e}");
            Err(e)
        }
    }
}

fn main() {
    let listener = match bind_server() {
        Ok(listener) => listener,
        Err(_) => exit(1)
    };
    let mut id: i32 = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn( move || { handle_client::Client::new_client(id, stream) });
                id += 1;
            },
            Err(e) => println!("Error: {e}")
        }
    };
}