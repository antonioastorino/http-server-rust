use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http_handler;
mod http_response_status;
use http_handler::http_response::*;
use http_handler::validator::*;
use http_response_status::*;

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        let response_status = handle_connection(stream.unwrap());
        let _response_header = http_response_header_create(&response_status);
        println!("{:?}", &response_status);
    }
}

fn handle_connection(mut stream: TcpStream) -> ResponseStatus {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
    let _body = split_text[1];
    let first_line_split = first_line.split(" ").collect::<Vec<&str>>();
    if first_line_split.len() != 3 {
        return ResponseStatus::BadRequest;
    }

    let request_type: RequestMethod = RequestMethod::from_str(first_line_split[0]);
    let request_address: &str = first_line_split[1];
    let request_version: &str = first_line_split[2];

    if request_type == RequestMethod::Unknown {
        return ResponseStatus::MethodNotAllowed;
    }
    match validate_version(request_version) {
        Ok(()) => println!("Request version: {}", &request_version),
        Err(()) => return ResponseStatus::HttpVersionNotSupported,
    };
    match validate_uri(request_address) {
        Ok(()) => println!("Request URI: {}", &request_address),
        Err(()) => return ResponseStatus::NotFound,
    };
    return ResponseStatus::Ok;
}
