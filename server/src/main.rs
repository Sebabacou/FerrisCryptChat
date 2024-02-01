mod handle_client;
mod server_manager;

use std::collections::HashMap;
use log::{debug, error, info};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

fn bind_server(addr: String) -> std::io::Result<TcpListener> {
    match TcpListener::bind(addr) {
        Ok(listener) => {
            debug!("Listening on {}", listener.local_addr().unwrap());
            Ok(listener)
        }
        Err(e) => {
            error!("Unable to bind: {e}");
            Err(e)
        }
    }
}

fn get_avaible_id(clients: Arc<Mutex<HashMap<u32, TcpStream>>>) -> u32 {
    let mut id = 1;
    while clients.lock().unwrap().contains_key(&id) {
        id += 1;
    }
    id
}

fn client_connection(listener: TcpListener, clients: Arc<Mutex<HashMap<u32, TcpStream>>>) {
    let _ = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients = Arc::clone(&clients);
                    let id = get_avaible_id(Arc::clone(&clients));
                    thread::spawn(move || handle_client::Client::new_client(id, stream, clients));
                }
                Err(e) => error!("Unable to connect: {e}"),
            }
        }
    });
}

fn main() {
    log4rs::init_file("server/log4rs.yml", Default::default()).unwrap();
    let listener = match bind_server("127.0.0.1:4242".to_string()) {
        Ok(listener) => listener,
        Err(_) => exit(1),
    };

    let clients: Arc<Mutex<HashMap<u32, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    client_connection(listener, Arc::clone(&clients));
    server_manager::ServerManager::server_handle(Arc::clone(&clients));
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
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput),
        }
    }
}
