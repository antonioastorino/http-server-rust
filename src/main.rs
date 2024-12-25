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
        let request_data: RequestData = handle_connection(&stream);
        /*
        let mut response_header: String;
        if request_data.http_version == RequestHttpVersion::Unknown {
                response_header_string = http_handler::response_header::create(ResponseStatus::HttpVersionNotSupported)
            }
        else if response_header.method == RequestData::Unknown {
            http_handler::response_header::create(ResponseStatus::MethodNotAllowed);
        }
        else if response_header.address_type == RequestAddressType::Unknonw {
            http_handler::response_header::create(ResponseStatus::NotFound);
        }
        */

        println!("{:?}", &request_data);
        //        stream.write(response_header.as_bytes()).unwrap();
        //        stream.flush().unwrap();
    }
}

fn handle_connection(mut stream: &TcpStream) -> RequestData {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    return parse_request(text.to_string());
}

fn parse_request(text: String) -> RequestData {
    let mut ret_request_data = RequestData::new();
    let split_text = text.lines().collect::<Vec<&str>>();
    let first_line = split_text[0];
    let first_line_split = first_line.split(" ").collect::<Vec<&str>>();

    ret_request_data.syntax = validate_syntax(&first_line_split);
    if ret_request_data.syntax == RequestSyntax::Unknown {
        return ret_request_data;
    }

    let request_method_str: &str = first_line_split[0];
    let request_address_str: &str = first_line_split[1];
    let request_version_str: &str = first_line_split[2];

    ret_request_data.method = validate_method(request_method_str);
    if ret_request_data.method == RequestMethod::Unknown {
        return ret_request_data;
    }

    ret_request_data.http_version = validate_version(request_version_str);
    if ret_request_data.http_version == RequestHttpVersion::Unknown {
        return ret_request_data;
    }

    ret_request_data.address_type = validate_address(request_address_str, &ret_request_data.method);
    if ret_request_data.address_type == RequestAddressType::Unknown {
        return ret_request_data;
    }
    return ret_request_data;
}

pub mod test {
    use crate::http_response_status::*;
    use crate::*;

    #[test]
    pub fn valid_addresses() {
        let request_data = parse_request(String::from("GET / HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Url);
        let request_data = parse_request(String::from("GET /index.html HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Url);
        let request_data = parse_request(String::from("GET /api/status HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Uri);
        let request_data = parse_request(String::from("POST /api/set HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Post);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Uri);
    }

    #[test]
    pub fn bad_request() {
        let request_data = parse_request(String::from("GET /missing_parameter\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
        let request_data = parse_request(String::from("GET /too many params\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
    }

    #[test]
    pub fn invalid_http_version() {
        let request_data = parse_request(String::from("GET /index.html anything\r\n"));
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
    }

    #[test]
    pub fn not_found() {
        let request_data = parse_request(String::from("GET /not_found HTTP/1.1\r\n"));
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
    }
}
