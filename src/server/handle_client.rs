use std::io::{BufRead, Write};
use crate::TcpStream;


pub struct Client {
    id: i32,
    stream: TcpStream,
}

impl Client {
    fn new(id: i32, stream: TcpStream) -> Client {
        Client{ id, stream }
    }

    pub fn new_client(id: i32, stream: TcpStream) {
        let mut client = Client::new(id, stream);
        println!("New connection: {0}, id: {1}", client.stream.peer_addr().unwrap(), client.id);
        client.message_handler();
    }

    fn answer_to_client(&mut self) {
        let msg = "ACK\0";
        println!("server answer");
        self.stream.write_all(msg.as_bytes()).expect(format!("Unable to send ACK to client {}", self.id).as_str());
    }

    fn message_handler(&mut self) {
        let mut buffer = Vec::new();
        let reader = self.stream.try_clone().expect(format!("Failed to clone stream for client {}", self.id).as_str());
        let mut reader = std::io::BufReader::new(reader);

        loop {
            match reader.read_until(b'\0', &mut buffer) {
                Ok(_) => {
                    if buffer.is_empty() {
                        println!("Client {0}, disconnected.", self.id);
                        return;
                    }
                    let msg = String::from_utf8_lossy(&buffer);
                    println!("Message from {0} : {msg}", self.id);
                    buffer.clear();
                    self.answer_to_client();
                }
                Err(e) => {
                    println!("Failed to read message from {0} : {e}", self.id);
                }
            }
        }
    }
}