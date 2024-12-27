use super::common::*;

pub fn from_address_to_path(address_str: &str) -> &'static str {
    let valid_addresses: [(&str, &str); 2] = [
        ("/api/status", "data/status.json"),
        ("/index.html", "www/index.html"),
    ];
    if address_str == "/" {
        return from_address_to_path("/index.html");
    };

    for (src, dest) in valid_addresses {
        if src == address_str {
            return dest;
        }
    }
    return "";
}

#[derive(Debug, PartialEq)]
pub struct RequestPayload {
    pub content_type: ContentType,
    pub content_size: ContentSize,
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
    pub fn new(text: String) -> Self {
        let mut ret_request_data = Self {
            syntax: RequestSyntax::Unknown,
            http_version: RequestHttpVersion::Unknown,
            method: RequestMethod::Unknown,
            address: "",
            payload: RequestPayload {
                content_type: ContentType::Unknown,
                content_size: 0,
            },
        };
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
        ret_request_data.http_version = validate_version(request_version_str);
        ret_request_data.address = from_address_to_path(request_address_str);
        // --- check for interesting parameters ---
        if split_text.len() > 1 {
            for line in split_text[1..].iter() {
                if line.starts_with("Content-Type: ") {
                    ret_request_data.payload.content_type = validate_content_type(line);
                }
                if line.starts_with("Content-Size: ") {
                    ret_request_data.payload.content_size = validate_content_size(line);
                }
            }
        }

        return ret_request_data;
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
    let split_line = content_type_str.split(": ").collect::<Vec<&str>>();
    if split_line.len() == 2 {
        return ContentType::from_content_type_str(split_line[1]);
    } else {
        return ContentType::Unknown;
    }
}

pub fn validate_content_size(content_size_str: &str) -> ContentSize {
    let split_line = content_size_str.split(": ").collect::<Vec<&str>>();
    if split_line.len() == 2 {
        match split_line[1].parse::<ContentSize>() {
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

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn valid_addresses() {
        let request_data = RequestHeader::new(
            vec![
                "GET / HTTP/1.1",
                "Content-Type: application/json",
                "Content-Size: 5",
                "\r\n",
            ]
            .join("\r\n"),
        );
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "www/index.html");
        assert_eq!(request_data.payload.content_type, ContentType::Json);
        assert_eq!(request_data.payload.content_size, 5);
        let request_data = RequestHeader::new(String::from("GET /index.html HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "www/index.html");
        let request_data = RequestHeader::new(String::from("GET /api/status HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "data/status.json");
        let request_data = RequestHeader::new(String::from("POST /api/set HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Post);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "");
    }

    #[test]
    pub fn bad_request() {
        let request_data = RequestHeader::new(String::from("GET /missing_parameter\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address, "");
        let request_data = RequestHeader::new(String::from("GET /too many params\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address, "");
    }

    #[test]
    pub fn invalid_http_version() {
        let request_data = RequestHeader::new(String::from("GET /index.html anything\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address, "www/index.html");
    }

    #[test]
    pub fn method_not_allowed() {
        let request_data = RequestHeader::new(String::from("PUT /index.html HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "www/index.html");
    }

    #[test]
    pub fn not_found() {
        let request_data = RequestHeader::new(String::from("POST /api/not_found HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Post);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "");
        let request_data = RequestHeader::new(String::from("GET /not_found HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address, "");
    }
}
