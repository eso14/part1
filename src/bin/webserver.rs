use std::net::{TcpListener, TcpStream};
use std::result::Result::Ok;
use std::io::{Read, Write};
use std::thread;

fn handle_client(stream: TcpStream) {
    let peer_addr = stream.peer_addr().expect("Could not get peer address");
    println!("Received connection from: {}", peer_addr);
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

