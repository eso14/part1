use std::net::{TcpListener, TcpStream};
use std::result::Result::Ok;
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().expect("Could not get peer address");
    let mut buffer = [0; 500];
    let mut request = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                let msg = match std::str::from_utf8(&buffer[..n]) {
                    Ok(s) => s,
                    Err(_) => "",
                };
                request.push_str(msg);
                if request.contains("\r\n\r\n") || request.contains("\n\n") {
                    break;
                }
            }
            Err(_e) => break,
        }
    }
    
    println!("Received request from {}:\n{}", peer_addr, request);
}

fn main() {
    let listener = TcpListener::bind("localhost:8888").expect("Could not bind to port 8888");
    println!("Server listening on port 8888...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

