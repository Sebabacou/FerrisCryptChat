use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::process::exit;
use std::str::FromStr;

enum StateAnswer {
    // TestPing = 00,
    ConnectionEstablished = 10,
    ConnectionClosed = 11,
    MessageSent = 20,
    // MessageNotSent = 21,
    BadDestination = 31,
}

trait Value {
    fn value(&self) -> u32;
}

impl Value for StateAnswer {
    fn value(&self) -> u32 {
        match self {
            StateAnswer::ConnectionEstablished => 10,
            StateAnswer::ConnectionClosed => 11,
            StateAnswer::MessageSent => 20,
            StateAnswer::BadDestination => 31,
        }
    }
}

impl FromStr for StateAnswer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "10\0" => Ok(StateAnswer::ConnectionEstablished),
            "11\0" => Ok(StateAnswer::ConnectionClosed),
            "20\0" => Ok(StateAnswer::MessageSent),
            "31\0" => Ok(StateAnswer::BadDestination),
            _ => Err(()),
        }
    }
}

struct Client {
    id: u32,
    addr: String,
    server: TcpStream,
}

impl Client {
    fn new(server: TcpStream) -> Self {
        Client {
            id: 0,
            addr: server.local_addr().unwrap().to_string(),
            server,
        }
    }
    pub fn new_client() {
        let server = match connect_to_server() {
            Ok(stream) => stream,
            Err(_) => exit(1),
        };
        let mut client = Client::new(server);
        client.wait_answer_from_server();
        client.send_message();
    }

    fn check_id(&mut self, id: u32) -> bool {
        if self.id == id {
            return true;
        }
        println!("Message not for client {}", self.id);
        false
    }

    fn split_id_from_status(&mut self, status: &str) -> (String, u32) {
        if let Some(index) = status.find("::") {
            let (id, status) = status.split_at(index);
            let status = status.trim_start_matches("::");
            if let Ok(id) = id.parse::<u32>() {
                return (status.to_string(), id);
            }
        }
        (status.to_string(), 0)
    }

    fn check_answer_from_server(&mut self, buffer: &mut Vec<u8>) {
        let answer = String::from_utf8_lossy(buffer);
        let (answer, id) = self.split_id_from_status(&answer);
        match answer.parse::<StateAnswer>() {
            Ok(state) => match state {
                StateAnswer::ConnectionEstablished => {
                    self.id = id;
                    println!(
                        "Connection established, my addr : {}, id : {}",
                        self.addr, self.id
                    );
                }
                StateAnswer::MessageSent => {
                    self.check_id(id)
                        .then(|| println!("Message Sent :: {}", StateAnswer::MessageSent.value()));
                }
                StateAnswer::BadDestination => {
                    self.check_id(id).then(|| {
                        println!("Bad Destination :: {}", StateAnswer::BadDestination.value())
                    });
                }
                StateAnswer::ConnectionClosed => {
                    self.check_id(id).then(|| {
                        println!(
                            "Connection closed by server :: {}",
                            StateAnswer::ConnectionClosed.value()
                        );
                    });
                }
            },
            Err(_) => println!("Answer not know"),
        }
    }

    fn temp_function(buffer: &mut Vec<u8>) {
        // TODO : get answer from client
        let answer = String::from_utf8_lossy(buffer);
        println!("message from someone : {answer}");
    }

    fn wait_answer_from_server(&mut self) {
        let mut buffer = Vec::new();
        let reader = self
            .server
            .try_clone()
            .expect("Client failed to clone stream");
        let mut reader = std::io::BufReader::new(reader);

        match reader.read_until(b'\0', &mut buffer) {
            Ok(_) => {
                buffer
                    .starts_with(b"<!STATUS!>")
                    .then(|| {
                        buffer.drain(..10);
                        self.check_answer_from_server(&mut buffer);
                    })
                    .or_else(|| Some(Client::temp_function(&mut buffer)));
            }
            Err(_) => {
                println!("Failed take answer of server");
            }
        }
    }

    fn send_message(&mut self) {
        loop {
            let mut msg = String::new();
            print!("$FCC_Client[{}] >_ ", self.id);
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut msg)
                .expect("Failed to read input");
            if msg.is_empty() || msg.trim() == "exit" {
                break;
            }
            msg.push_str("\0");
            self.server
                .write_all(msg.as_bytes())
                .expect("Failed to send message.");
            self.wait_answer_from_server();
        }
    }
}

fn connect_to_server() -> std::io::Result<TcpStream> {
    match TcpStream::connect("127.0.0.1:4242") {
        Ok(stream) => Ok(stream),
        Err(e) => {
            println!("Unable to connect: {e}");
            Err(e)
        }
    }
}

fn main() {
    Client::new_client();
}
