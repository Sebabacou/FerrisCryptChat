use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use log::{error, info};


pub struct ServerManager {
    clients: Arc<Mutex<HashMap<u32, TcpStream>>>
}

impl ServerManager {

    fn show_clients(&self) {
        let clients = self.clients.lock().unwrap();
        println!("Clients connected:");
        for (id, stream) in clients.iter() {
            println!("-> id: {0}, addr: {1}", id, stream.peer_addr().unwrap());
        }
    }

    pub fn server_handle(clients: Arc<Mutex<HashMap<u32, TcpStream>>>) {
        let server_manager = ServerManager { clients };
        loop {
            let mut msg = String::new();
            print!("$FCC_Server >_ ");
            std::io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut msg) {
                Ok(_) => (),
                Err(e) => {
                    error!("Unable to read stdin: {e}");
                    panic!();
                },
            }
            if msg.trim() == "show clients" {
                server_manager.show_clients();
            }
            if msg.is_empty() || msg.trim() == "exit" {
                info!("Shutdown server");
                break;
            }
        }
    }
}