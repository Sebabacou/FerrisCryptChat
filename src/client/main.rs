use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::process::exit;

fn connect_to_server() -> std::io::Result<TcpStream> {
    match TcpStream::connect("127.0.0.1:4242") {
        Ok(stream) =>  {
            println!("Connected to server on {}", stream.local_addr().unwrap());
            Ok(stream)
        },
        Err(e) => {
            println!("Unable to connect: {e}");
            Err(e)
        }
    }
}

fn wait_answer_from_server(stream: &mut TcpStream) {
    let mut buffer = Vec::new();
    let reader = stream.try_clone().expect("Client failed to clone stream");
    let mut reader = std::io::BufReader::new(reader);
    match reader.read_until(b'\0', &mut buffer) {
        Ok(_) => {
            println!("Server get the message");
        },
        Err(_) => {
            println!("Failed take answer of server");
        }
    }
}
fn send_message(stream: &mut TcpStream) {
    loop {
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg).expect("Failed to read input");
        if msg.is_empty() || msg.trim() == "exit" {
            break;
        }
        msg.push_str("\0");
        stream.write_all(msg.as_bytes()).expect("Failed to send message.");
        wait_answer_from_server(stream);
    }
}

fn main() {
    let mut stream = match connect_to_server() {
        Ok(stream) => stream,
        Err(_) => exit(1)
    };
    send_message(&mut stream);
}