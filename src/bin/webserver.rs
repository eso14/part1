use std::net::{TcpListener, TcpStream};
use std::result::Result::Ok;
use std::io::{Read, Write};
use std::thread;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};


struct RequestCounter {
    total_requests: usize,
    valid_requests: usize,
}


fn handle_client(mut stream: TcpStream, counter: Arc<Mutex<RequestCounter>>) {
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
    
    println!("Client IP Address: {} \n Read {} bytes from{} \n :\n{}", peer_addr, request.len(), peer_addr, request);

    let mut counter_lock = counter.lock().unwrap();
    counter_lock.total_requests += 1;
    drop(counter_lock);

    let requested_file = request.lines()
    .next()
    .and_then(|line| line.split_whitespace().nth(1))
    .unwrap_or("/");
    
    let curr_dir = std::env::current_dir().expect("Failed to get current directory");
    let requested_path = Path::new(requested_file.strip_prefix("/").unwrap_or(requested_file));
    let full_path = curr_dir.join(requested_path);

    if !full_path.exists() {
        let response = "HTTP/1.1 404 Not Found\r\n\r\nFile not found";
        stream.write_all(response.as_bytes()).expect("Failed to send response");
        return;
    }
    if full_path.is_dir() {
        let response = "HTTP/1.1 404 Not Found\r\n\r\nDirectories not supported";
        stream.write_all(response.as_bytes()).expect("Failed to send response");
        return;
    }

    
    let mut counter_lock = counter.lock().unwrap();
    counter_lock.valid_requests += 1;
    drop(counter_lock);
    

    match fs::read_to_string(&full_path) {
        Ok(contents) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(), contents
            );
            stream.write_all(response.as_bytes()).expect("Failed to send response");
        }
        Err(_) => {
            let response = "Failed to read file";
            stream.write_all(response.as_bytes()).expect("Failed to send response");
        }
    }

    stream.flush().expect("Failed to flush stream");

    let counter_lock = counter.lock().unwrap();
    println!("Total Requests: {}, Valid Requests: {}", counter_lock.total_requests, counter_lock.valid_requests);
}

fn main() {
    let listener = TcpListener::bind("localhost:8888").expect("Could not bind to port 8888");
    println!("Server listening on port 8888...");

    let counter = Arc::new(Mutex::new(RequestCounter { total_requests: 0, valid_requests: 0 }));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let counter_clone = Arc::clone(&counter);
                thread::spawn(move || handle_client(stream, counter_clone));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

