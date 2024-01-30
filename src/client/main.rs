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

fn send_message(stream: &mut TcpStream) {
    loop { //TODO : check ACK before continue
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg).expect("Failed to read input");
        if msg.is_empty() || msg.trim() == "exit" {
            break;
        }
        msg.push_str("\0");
        stream.write_all(msg.as_bytes()).expect("Failed to send message.");
        println!("msg send");
    }
}

fn main() {
    let mut stream = match connect_to_server() {
        Ok(stream) => stream,
        Err(_) => exit(1)
    };
    send_message(&mut stream);
}