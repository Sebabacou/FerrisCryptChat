mod handle_client;

use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;

fn bind_server(addr: String) -> std::io::Result<TcpListener> {
    match TcpListener::bind(addr) {
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
    let listener = match bind_server("127.0.0.1:4242".to_string()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_server() {
        let listener = bind_server("127.0.0.1:4242".to_string()).unwrap();
        assert_eq!(listener.local_addr().unwrap().to_string(), "127.0.0.1:4242");
    }

    #[test]
    fn unable_to_bind_server() {
        match bind_server("0.0.0.0".to_string()) {
            Ok(_) => panic!("Should not be able to bind"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput)
        }
    }
}
