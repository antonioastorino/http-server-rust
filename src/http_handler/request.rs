use super::common::*;
use super::files::*;
const BUFF_READ_SIZE: usize = 32768;

#[derive(Debug, PartialEq)]
pub struct RequestPayload {
    pub content_type: ContentType,
    pub content_length: ContentLength,
}

#[derive(Debug, PartialEq)]
pub enum RequestSyntax {
    Known,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum RequestHttpVersion {
    Http11,
    Unknown,
}

impl RequestHttpVersion {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Http11 => "HTTP/1.1",
            Self::Unknown => "",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct RequestHeader {
    pub syntax: RequestSyntax,
    pub http_version: RequestHttpVersion,
    pub method: RequestMethod,
    pub address: &'static str,
    pub payload: RequestPayload,
}

impl RequestHeader {
    pub fn new(text: &String) -> Self {
        let mut ret_request_header = Self {
            syntax: RequestSyntax::Unknown,
            http_version: RequestHttpVersion::Unknown,
            method: RequestMethod::Unknown,
            address: "",
            payload: RequestPayload {
                content_type: ContentType::Unknown,
                content_length: 0,
            },
        };
        let split_text = text.lines().collect::<Vec<&str>>();
        let first_line = split_text[0];
        let first_line_split = first_line.split(" ").collect::<Vec<&str>>();

        ret_request_header.syntax = validate_syntax(&first_line_split);
        if ret_request_header.syntax == RequestSyntax::Unknown {
            return ret_request_header;
        }

        let request_method_str: &str = first_line_split[0];
        let request_address_str: &str = first_line_split[1];
        let request_version_str: &str = first_line_split[2];

        ret_request_header.method = validate_method(request_method_str);
        ret_request_header.http_version = validate_version(request_version_str);
        ret_request_header.address = from_address_to_path(request_address_str);
        // --- check for interesting parameters ---
        if split_text.len() > 1 {
            for line in split_text[1..].iter() {
                if line.to_uppercase().starts_with("CONTENT-TYPE:") {
                    ret_request_header.payload.content_type = validate_content_type(line);
                }
                if line.to_uppercase().starts_with("CONTENT-LENGTH:") {
                    ret_request_header.payload.content_length = validate_content_length(line);
                }
            }
        }
        return ret_request_header;
    }
}

pub fn validate_syntax(input: &Vec<&str>) -> RequestSyntax {
    if input.len() != 3 {
        return RequestSyntax::Unknown;
    }
    return RequestSyntax::Known;
}

pub fn validate_method(input: &str) -> RequestMethod {
    match input {
        "GET" => RequestMethod::Get,
        "POST" => RequestMethod::Post,
        _ => return RequestMethod::Unknown,
    }
}

pub fn validate_version(version: &str) -> RequestHttpVersion {
    if version == "HTTP/1.1" {
        return RequestHttpVersion::Http11;
    }
    return RequestHttpVersion::Unknown;
}

pub fn validate_content_type(content_type_str: &str) -> ContentType {
    let split_line = content_type_str.split(":").collect::<Vec<&str>>();
    if split_line.len() == 2 {
        return ContentType::from_content_type_str(split_line[1].trim());
    } else {
        return ContentType::Unknown;
    }
}

pub fn validate_content_length(content_length_str: &str) -> ContentLength {
    let split_line = content_length_str.split(":").collect::<Vec<&str>>();
    if split_line.len() == 2 {
        match split_line[1].trim().parse::<ContentLength>() {
            Ok(value) => {
                return value;
            }
            Err(_) => {
                return 0;
            }
        }
    }
    return 0;
}

pub struct RequestBody {}

impl RequestBody {
    pub fn process(
        request_header: &RequestHeader,
        reader: &mut std::io::BufReader<std::net::TcpStream>,
    ) {
        use std::io::prelude::Read;
        use std::io::Write;
        if request_header.method == RequestMethod::Post && request_header.payload.content_length > 0
        {
            let mut capacity: usize = request_header.payload.content_length.try_into().unwrap();
            let mut bytes_read: usize;
            let mut body: [u8; BUFF_READ_SIZE] = [0; BUFF_READ_SIZE];
            let out_file_path = format!("artifacts/{:?}-data", request_header.payload.content_type);
            let mut out_file = std::fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&out_file_path)
                .unwrap();
            loop {
                bytes_read = reader.read(&mut body).unwrap();
                println!("read {} - remaining {}", bytes_read, capacity);
                if request_header.payload.content_type.is_binary() {
                    out_file.write_all(&body[0..bytes_read]).unwrap();
                } else {
                    write!(
                        &mut out_file,
                        "{}",
                        std::str::from_utf8(&body[0..bytes_read]).unwrap()
                    )
                    .unwrap();
                }
                if capacity > bytes_read {
                    capacity -= bytes_read;
                } else if capacity == bytes_read {
                    println!("Upload completed");
                    break;
                } else {
                    panic!("Received more data than expected! This should not happen");
                }
            }
        }
    }
}
#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn valid_addresses() {
        let request_header = RequestHeader::new(
            &vec![
                "GET / HTTP/1.1",
                "Content-Type: application/json",
                "Content-Length: 5",
                "\r\n",
            ]
            .join("\r\n"),
        );
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "www/index.html");
        assert_eq!(request_header.payload.content_type, ContentType::Json);
        assert_eq!(request_header.payload.content_length, 5);

        let request_header = RequestHeader::new(&String::from("GET /index.html HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "www/index.html");

        let request_header = RequestHeader::new(&String::from("GET /img/test.png HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "www/img/test.png");

        let request_header = RequestHeader::new(&String::from("GET /api/status HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "data/status.json");

        let request_header = RequestHeader::new(&String::from("POST /api/set HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Post);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "");
    }

    #[test]
    pub fn bad_request() {
        let request_header = RequestHeader::new(&String::from("GET /missing_parameter\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Unknown);
        assert_eq!(request_header.method, RequestMethod::Unknown);
        assert_eq!(request_header.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_header.address, "");
        let request_header = RequestHeader::new(&String::from("GET /too many params\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Unknown);
        assert_eq!(request_header.method, RequestMethod::Unknown);
        assert_eq!(request_header.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_header.address, "");
    }

    #[test]
    pub fn invalid_http_version() {
        let request_header = RequestHeader::new(&String::from("GET /index.html anything\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_header.address, "www/index.html");
    }

    #[test]
    pub fn method_not_allowed() {
        let request_header = RequestHeader::new(&String::from("PUT /index.html HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Unknown);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "www/index.html");
    }

    #[test]
    pub fn not_found() {
        let request_header = RequestHeader::new(&String::from("POST /api/not_found HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Post);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "");
        let request_header = RequestHeader::new(&String::from("GET /not_found HTTP/1.1\r\n"));
        assert_eq!(request_header.syntax, RequestSyntax::Known);
        assert_eq!(request_header.method, RequestMethod::Get);
        assert_eq!(request_header.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_header.address, "");
    }
}
