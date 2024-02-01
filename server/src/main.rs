mod handle_client;

use std::io::Write;
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

fn client_connection(listener: TcpListener) {
    let _ = thread::spawn(move || {
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
    });
}

fn main() {
    let listener = match bind_server("127.0.0.1:4242".to_string()) {
        Ok(listener) => listener,
        Err(_) => exit(1),
    };

    client_connection(listener);
    loop {
        let mut msg = String::new();
        print!("$FCC_Server >_ ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read input");
        if msg.is_empty() || msg.trim() == "exit" {
            break;
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
        match bind_server("127.0.0.1:65536".to_string()) {
            Ok(_) => panic!("Should not be able to bind"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::AddrNotAvailable)
        }
    }
}
