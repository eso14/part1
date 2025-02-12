use std::fs;
use std::sync::Arc;
use std::{env, net::TcpStream};
use std::io::{BufReader, Read, Write};
use rustls;
use webpki_roots;
use anyhow;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: webget url");
    } 
    let url = &args[1];
}

fn parse_url(url : String) -> Option<String, String>{
    let url = url.strip_prefix("https://");


}

fn send_message(host: &str, port: usize, message: &str) -> anyhow::Result<()> {
    // Obtain standard set of trusted TLS certificates
    let root_store = rustls::RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };
    
    // Use the trusted set above; do not offer a certificate on the
    // client side, as the client is not claiming to be trusted.
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    // Set up ServerName object
    let server_name = host.to_string().try_into().unwrap();
    
    // Create TCP connection 
    let mut tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    
    // Create TLS connection
    let mut connector = rustls::ClientConnection::new(Arc::new(config), server_name)?;
    
    // Create I/O stream
    let mut stream = rustls::Stream::new(&mut connector, &mut tcp);
    write!(stream, "{message}")?;
    
    // TODO: ****Write code here to read and process the response from the socket.****
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_to_string(&mut response)?;

    let mut lines = response.lines();
    while let Some(line) = lines.next(){
        if line.is_empty(){
            break;
        }
        println!("{line}");
    }

    let filename = host.replace('.', "_") + ".html";
    fs::write(&filename, lines.collect::<Vec<&str>>().join("\n"));
    println!("Saved to {filename} successfully");
    Ok(())
}