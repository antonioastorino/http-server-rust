use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
enum RequestType {
    GET,
    POST,
    UNKNOWN,
}

impl RequestType {
    fn from_str(input: &str) -> RequestType {
        match input {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            &_ => RequestType::UNKNOWN,
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    let header = text.lines().collect::<Vec<&str>>()[0];
    let request_type: RequestType = RequestType::from_str(header.split(" ").collect::<Vec<&str>>()[0]);
    println!("{:?}", request_type);
}
