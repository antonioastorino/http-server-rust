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
        let response_header = match handle_connection(&stream) {
            Ok(_) => {
                let tmp = ResponseStatus::Ok;
                http_handler::response_header::create(&tmp)
            }
            Err(status) => http_handler::response_header::create(&status),
        };
        println!("{}", &response_header);
        stream.write(response_header.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_connection(mut stream: &TcpStream) -> Result<RequestData, ResponseStatus> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    return parse_request(text.to_string());
}

fn parse_request(text: String) -> Result<RequestData, ResponseStatus> {
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
    let first_line_split = first_line.split(" ").collect::<Vec<&str>>();
    if first_line_split.len() != 3 {
        return Err(ResponseStatus::BadRequest);
    }

    let request_method_str: &str = first_line_split[0];
    let request_address_str: &str = first_line_split[1];
    let request_version_str: &str = first_line_split[2];

    let request_method: RequestMethod = match validate_method(request_method_str) {
        Ok(method) => method,
        Err(()) => {
            return Err(ResponseStatus::MethodNotAllowed);
        }
    };

    match validate_version(request_version_str) {
        Ok(()) => println!("Request version: {}", &request_version_str),
        Err(()) => {
            return Err(ResponseStatus::HttpVersionNotSupported);
        }
    };

    let request_address_type: RequestAddressType =
        match validate_address(request_address_str, &request_method) {
            Ok(address_type) => address_type,
            Err(()) => {
                return Err(ResponseStatus::NotFound);
            }
        };

    println!(
        "Address type: {:?} - Method: {:?}",
        &request_address_type, &request_method
    );
    return Ok(RequestData {
        method: request_method,
        address_type: request_address_type,
    });
}

pub mod test {
    use crate::http_response_status::*;
    use crate::*;

    #[test]
    pub fn valid_addresses() {
        let request_data = parse_request(String::from("GET / HTTP/1.1\r\n")).unwrap();
        assert_eq!(request_data.method, RequestMethod::Get);
        let request_data = parse_request(String::from("GET /index.html HTTP/1.1\r\n")).unwrap();
        assert_eq!(request_data.method, RequestMethod::Get);
        let request_data = parse_request(String::from("GET /api/status HTTP/1.1\r\n")).unwrap();
        assert_eq!(request_data.method, RequestMethod::Get);
        let request_data = parse_request(String::from("POST /api/set HTTP/1.1\r\n")).unwrap();
        assert_eq!(request_data.method, RequestMethod::Post);
    }

    #[test]
    pub fn bad_request() {
        let status: ResponseStatus =
            parse_request(String::from("GET /missing_parameter\r\n")).unwrap_err();
        assert_eq!(status, ResponseStatus::BadRequest);
        let status: ResponseStatus =
            parse_request(String::from("GET /too many params\n")).unwrap_err();
        assert_eq!(status, ResponseStatus::BadRequest);
    }

    #[test]
    pub fn invalid_http_version() {
        let status: ResponseStatus =
            parse_request(String::from("GET /index.html anything\r\n")).unwrap_err();
        assert_eq!(status, ResponseStatus::HttpVersionNotSupported);
    }

    #[test]
    pub fn not_found() {
        let status: ResponseStatus =
            parse_request(String::from("GET /not_found HTTP/1.1\r\n")).unwrap_err();
        assert_eq!(status, ResponseStatus::NotFound);
    }
}
