use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http_handler;
use http_handler::common_handler::RequestType;

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        match handle_connection(stream.unwrap()) {
            Ok(()) => println!("Ok"),
            Err(string) => println!("{}", string),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), &'static str> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
    let body = split_text[1];
    let first_line_split = first_line.split(" ").collect::<Vec<&str>>();
    if first_line_split.len() != 3 {
        return Err("Invalid first line");
    }

    let request_type: RequestType = RequestType::from_str(first_line_split[0]);
    let request_address: &str = first_line_split[1];
    let request_version: &str = first_line_split[2];
    println!("{:?}", &request_version);
    match request_type {
        RequestType::GET => http_handler::get_handler::validate(request_address)?,
        RequestType::POST => handle_post_request(&body),
        RequestType::UNKNOWN => todo!(),
    }
    return Ok(());
}

fn handle_post_request(body: &str) {
    println!("{:?}", body);
}
