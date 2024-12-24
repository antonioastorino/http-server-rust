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

    let request_method_str: &str = first_line_split[0];
    let request_address_str: &str = first_line_split[1];
    let request_version_str: &str = first_line_split[2];

    let request_method: RequestMethod = match validate_method(request_method_str) {
        Ok(method) => method,
        Err(()) => {
            return ResponseStatus::MethodNotAllowed;
        }
    };

    match validate_version(request_version_str) {
        Ok(()) => println!("Request version: {}", &request_version_str),
        Err(()) => return ResponseStatus::HttpVersionNotSupported,
    };

    let request_address_type: RequestAddressType =
        match validate_address(request_address_str, &request_method) {
            Ok(address_type) => address_type,
            Err(()) => {
                return ResponseStatus::NotFound;
            }
        };
    println!("Address type: {:?} - Method: {:?}", request_address_type, &request_method);
    return ResponseStatus::Ok;
}
