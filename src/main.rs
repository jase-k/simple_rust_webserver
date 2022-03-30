use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream); //taking ownership
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024]; //An array of 1024 0's

    stream.read(&mut buffer).unwrap(); //Takes in a buffer and must mutate it

    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); //translate a buffer to readable text

    let get = b"GET / HTTP/1.1\r\n"; // Will match a buffer which is a byte array

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let html = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
