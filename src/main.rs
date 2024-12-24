use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http_handler;
mod http_response_status;
use http_handler::request_validator::*;
use http_response_status::*;

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for stream in listener.incoming() {
        let response_status = handle_connection(stream.unwrap());
        let response_header = http_handler::response_header::create(&response_status);
        println!("{}", &response_header);
    }
}

fn handle_connection(mut stream: TcpStream) -> ResponseStatus {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    return parse_request(text.to_string());
}

fn parse_request(text: String) -> ResponseStatus {
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
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
    println!(
        "Address type: {:?} - Method: {:?}",
        request_address_type, &request_method
    );
    return ResponseStatus::Ok;
}

pub mod test {
    use crate::http_response_status::*;
    use crate::*;

    #[test]
    pub fn valid_addresses() {
        let status: ResponseStatus = parse_request(String::from("GET / HTTP/1.1\r\n"));
        assert_eq!(status, ResponseStatus::Ok);
        let status: ResponseStatus = parse_request(String::from("GET /index.html HTTP/1.1\r\n"));
        assert_eq!(status, ResponseStatus::Ok);
    }

    #[test]
    pub fn bad_request() {
        let status: ResponseStatus = parse_request(String::from("GET /missing_parameter\r\n"));
        assert_eq!(status, ResponseStatus::BadRequest);
        let status: ResponseStatus = parse_request(String::from("GET /too many params\n"));
        assert_eq!(status, ResponseStatus::BadRequest);
    }

    #[test]
    pub fn invalid_http_version() {
        let status: ResponseStatus = parse_request(String::from("GET /index.html anything\r\n"));
        assert_eq!(status, ResponseStatus::HttpVersionNotSupported);
    }

    #[test]
    pub fn not_found() {
        let status: ResponseStatus = parse_request(String::from("GET /not_found HTTP/1.1\r\n"));
        assert_eq!(status, ResponseStatus::NotFound);
    }
}
