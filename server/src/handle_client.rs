use crate::TcpStream;
use std::io::{BufRead, Write};

const ALL: u32 = 0;
const SERVER: u32 = 4294967295;

macro_rules! answer {
    ($msg:expr, $id:expr) => {
        format!("<!STATUS!>{}::{}\0", $id, $msg as u32)
    };
}

enum StateAnswer {
    // TestPing = 00,
    ConnectionEstablished = 10,
    // ConnectionClosed = 11,
    MessageSent = 20,
    // MessageNotSent = 21,
    BadDestination = 31,
}

pub struct Client {
    id: u32,
    stream: TcpStream,
    dest: Option<u32>,
}

impl Client {
    fn new(id: u32, stream: TcpStream) -> Client {
        let dest: Option<u32> = None;
        Client { id, stream, dest }
    }

    pub fn new_client(id: u32, stream: TcpStream) {
        let mut client = Client::new(id, stream);
        println!(
            "New connection: {0}, id: {1}",
            client.stream.peer_addr().unwrap(),
            client.id
        );
        client.msg_state_client(StateAnswer::ConnectionEstablished);
        client.message_handler();
    }

    fn msg_state_client(&mut self, msg: StateAnswer) {
        let msg = answer!(msg, self.id);
        match self.stream.write_all(msg.as_bytes()) {
            Ok(_) => return,
            Err(e) => println!("Failed to send state to client {0} : {e}", self.id),
        }
    }

    fn check_dest(&mut self, dest: &str) {
        let dest = dest.trim_end_matches("::");
        self.dest = match dest {
            "all" => Some(ALL),
            "server" => Some(SERVER),
            _ => match dest.parse::<u32>() {
                Ok(dest) => Some(dest),
                Err(_) => None,
            },
        };
        match self.dest {
            Some(value) => println!("Msg dest is : {value}"),
            None => println!("Invalid destination"),
        }
    }

    fn get_destination(&mut self, buffer: Vec<u8>) -> String {
        let msg = String::from_utf8_lossy(&buffer);
        let msg = match msg.find("::") {
            Some(index) => {
                let (dest, msg) = msg.split_at(index + "::".len());
                self.check_dest(dest);
                msg
            }
            None => {
                println!("No destination");
                self.dest = None;
                &msg
            }
        };
        msg.to_string()
    }

    fn message_handler(&mut self) {
        let mut buffer = Vec::new();
        let reader = self
            .stream
            .try_clone()
            .expect(format!("Failed to clone stream for client {}", self.id).as_str());
        let mut reader = std::io::BufReader::new(reader);

        loop {
            match reader.read_until(b'\0', &mut buffer) {
                Ok(_) => {
                    if buffer.is_empty() {
                        println!("Client {0}, disconnected.", self.id);
                        return;
                    }
                    let msg = self.get_destination(buffer.clone());
                    match self.dest {
                        Some(_) => self.msg_state_client(StateAnswer::MessageSent),
                        None => self.msg_state_client(StateAnswer::BadDestination),
                    };
                    print!("Message from {0} : {msg}", self.id);
                    buffer.clear();
                }
                Err(e) => {
                    println!("Failed to read message from {0} : {e}", self.id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            answer!(StateAnswer::ConnectionEstablished, 1),
            "<!STATUS!>1::10\0"
        );
        assert_eq!(answer!(StateAnswer::MessageSent, 1), "<!STATUS!>1::20\0");
        assert_eq!(answer!(StateAnswer::BadDestination, 1), "<!STATUS!>1::31\0");
    }
}
