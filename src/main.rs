use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::str;

fn main() -> std::io::Result<()> {
    // Bind the server to a local address and port
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server running on http://127.0.0.1:7878");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

// Function to handle each incoming connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Read the request from the stream
    match stream.read(&mut buffer) {
        Ok(size) => {
            // Convert the request to a string
            let request = str::from_utf8(&buffer[..size]).unwrap();
            println!("Request: {}", request);

            // Parse the request and determine the response
            let response = if request.starts_with("GET / HTTP/1.1") {
                // Respond with an HTML page for the root path
                let contents = fs::read_to_string("index.html").unwrap_or_else(|_| {
                    String::from("<h1>404 Not Found</h1>")
                });
                format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents)
            } else {
                // Respond with 404 for other paths
                String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>")
            };

            // Write the response back to the stream
            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Failed to write response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}