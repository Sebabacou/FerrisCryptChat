use std::io::{BufRead, Write};
use crate::TcpStream;

const ALL: u32 = 0;
const SERVER: u32 = 4294967295;

pub struct Client {
    id: u32,
    stream: TcpStream,
    dest: Option<u32>,
}

impl Client {
    fn new(id: u32, stream: TcpStream) -> Client {
        let dest: Option<u32> = None;
        Client{ id, stream, dest }
    }

    pub fn new_client(id: u32, stream: TcpStream) {
        let mut client = Client::new(id, stream);
        println!("New connection: {0}, id: {1}", client.stream.peer_addr().unwrap(), client.id);
        client.message_handler();
    }

    fn answer_to_client(&mut self) {
        let msg = match self.dest {
            Some(_dest) => "ACK\0",
            None => "BAD_DEST\0"
        };
        self.stream.write_all(msg.as_bytes()).expect(format!("Unable to send ACK to client {}", self.id).as_str());
    }

    fn check_dest(&mut self, dest: &str) {
        let dest = dest.trim_end_matches("::");
        self.dest = match dest {
            "all" => Some(ALL),
            "server" => Some(SERVER),
            _ => {
                match dest.parse::<u32>() {
                    Ok(dest) => Some(dest),
                    Err(_) => None,
                }
            }
        };
        match self.dest {
            Some(value) => println!("Msg dest is : {value}"),
            None => println!("Invalid destination")
        }
    }
    fn get_destination(&mut self, buffer: &Vec<u8>) -> String{ // TODO : all = 0 | server = 4 294 967 295
        let msg = String::from_utf8_lossy(&buffer);
        let msg = match msg.find("::") {
            Some(index) => {
                let (dest, msg) = msg.split_at(index + "::".len());
                self.check_dest(dest);
                msg
            }
            None => {
                println!("No destination");
                //TODO : Tell to client not a valid dest
                self.dest = None;
                &msg
            }

        };
        msg.to_string()
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
                    let msg = self.get_destination(&buffer);
                    print!("Message from {0} : {msg}", self.id);
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