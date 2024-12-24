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
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let response_contents = handle_connection(&stream);
        let response_header = http_handler::response_header::create(&response_contents.status);
        println!("{}", &response_header);
        stream.write(response_header.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_connection(mut stream: &TcpStream) -> ResponseContents {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    return parse_request(text.to_string());
}

fn parse_request(text: String) -> ResponseContents {
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
    let first_line_split = first_line.split(" ").collect::<Vec<&str>>();
    if first_line_split.len() != 3 {
        return ResponseContents {
            status: ResponseStatus::BadRequest,
            path: "",
            content_type: ContentType::Empty,
        };
    }

    let request_method_str: &str = first_line_split[0];
    let request_address_str: &str = first_line_split[1];
    let request_version_str: &str = first_line_split[2];

    let request_method: RequestMethod = match validate_method(request_method_str) {
        Ok(method) => method,
        Err(()) => {
            return ResponseContents {
                status: ResponseStatus::MethodNotAllowed,
                path: "",
                content_type: ContentType::Empty,
            };
        }
    };

    match validate_version(request_version_str) {
        Ok(()) => println!("Request version: {}", &request_version_str),
        Err(()) => {
            return ResponseContents {
                status: ResponseStatus::HttpVersionNotSupported,
                path: "",
                content_type: ContentType::Empty,
            };
        }
    };

    let request_address_type: RequestAddressType =
        match validate_address(request_address_str, &request_method) {
            Ok(address_type) => address_type,
            Err(()) => {
                return ResponseContents {
                    status: ResponseStatus::NotFound,
                    path: "",
                    content_type: ContentType::Empty,
                };
            }
        };
    println!(
        "Address type: {:?} - Method: {:?}",
        &request_address_type, &request_method
    );
    return ResponseContents {
        status: ResponseStatus::Ok,
        path: "should be taken from a static list",
        content_type: ContentType::Empty,
    };
}

pub mod test {
    use crate::http_response_status::*;
    use crate::*;

    #[test]
    pub fn valid_addresses() {
        let status: ResponseStatus = parse_request(String::from("GET / HTTP/1.1\r\n")).status;
        assert_eq!(status, ResponseStatus::Ok);
        let status: ResponseStatus =
            parse_request(String::from("GET /index.html HTTP/1.1\r\n")).status;
        assert_eq!(status, ResponseStatus::Ok);
    }

    #[test]
    pub fn bad_request() {
        let status: ResponseStatus =
            parse_request(String::from("GET /missing_parameter\r\n")).status;
        assert_eq!(status, ResponseStatus::BadRequest);
        let status: ResponseStatus = parse_request(String::from("GET /too many params\n")).status;
        assert_eq!(status, ResponseStatus::BadRequest);
    }

    #[test]
    pub fn invalid_http_version() {
        let status: ResponseStatus =
            parse_request(String::from("GET /index.html anything\r\n")).status;
        assert_eq!(status, ResponseStatus::HttpVersionNotSupported);
    }

    #[test]
    pub fn not_found() {
        let status: ResponseStatus =
            parse_request(String::from("GET /not_found HTTP/1.1\r\n")).status;
        assert_eq!(status, ResponseStatus::NotFound);
    }
}
