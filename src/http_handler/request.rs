const VALID_API_GET: [&'static str; 1] = ["/api/status"];
const VALID_API_POST: [&'static str; 1] = ["/api/set"];
const VALID_PAGES: [&'static str; 2] = ["/", "/index.html"];

pub fn from_address_to_path(path_str: &str) -> &'static str {
    if ["/", "/index.html"].contains(&path_str) {
        return "www/index.html";
    }
    if path_str == "/api/status" {
        return "data/status.json";
    }
    for post_str in VALID_API_POST {
        if post_str == path_str {
            return post_str;
        }
    }

    return "";
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
pub enum RequestAddressType {
    Uri,
    Url,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub syntax: RequestSyntax,
    pub http_version: RequestHttpVersion,
    pub method: RequestMethod,
    pub address_type: RequestAddressType,
    pub address: &'static str,
}

impl Request {
    pub fn new(text: String) -> Self {
        let mut ret_request_data = Self {
            syntax: RequestSyntax::Unknown,
            http_version: RequestHttpVersion::Unknown,
            method: RequestMethod::Unknown,
            address_type: RequestAddressType::Unknown,
            address: "",
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
        ret_request_data.address_type =
            validate_address(request_address_str, &ret_request_data.method);
        ret_request_data.address = from_address_to_path(request_address_str);
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

pub fn validate_address(address: &str, method: &RequestMethod) -> RequestAddressType {
    match method {
        RequestMethod::Get => {
            if VALID_API_GET.contains(&address) {
                return RequestAddressType::Uri;
            }
            if VALID_PAGES.contains(&address) {
                return RequestAddressType::Url;
            }
            return RequestAddressType::Unknown;
        }
        RequestMethod::Post => {
            if VALID_API_POST.contains(&address) {
                return RequestAddressType::Uri;
            }
            return RequestAddressType::Unknown;
        }
        RequestMethod::Unknown => {
            return RequestAddressType::Unknown;
        }
    }
}

pub fn validate_version(version: &str) -> RequestHttpVersion {
    if version == "HTTP/1.1" {
        return RequestHttpVersion::Http11;
    }
    return RequestHttpVersion::Unknown;
}

pub mod test {
    #[test]
    pub fn valid_addresses() {
        let request_data = Request::new(String::from("GET / HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Url);
        assert_eq!(request_data.address, "www/index.html");
        let request_data = Request::new(String::from("GET /index.html HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Url);
        assert_eq!(request_data.address, "www/index.html");
        let request_data = Request::new(String::from("GET /api/status HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Uri);
        assert_eq!(request_data.address, "data/status.json");
        let request_data = Request::new(String::from("POST /api/set HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Post);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Uri);
        assert_eq!(request_data.address, "/api/set");
    }

    #[test]
    pub fn bad_request() {
        let request_data = Request::new(String::from("GET /missing_parameter\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
        assert_eq!(request_data.address, "");
        let request_data = Request::new(String::from("GET /too many params\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Unknown);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
        assert_eq!(request_data.address, "");
    }

    #[test]
    pub fn invalid_http_version() {
        let request_data = Request::new(String::from("GET /index.html anything\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Unknown);
        assert_eq!(request_data.address_type, RequestAddressType::Url);
        assert_eq!(request_data.address, "www/index.html");
    }

    #[test]
    pub fn method_not_allowed() {
        let request_data = Request::new(String::from("PUT /index.html HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Unknown);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
        assert_eq!(request_data.address, "www/index.html");
    }

    #[test]
    pub fn not_found() {
        let request_data = Request::new(String::from("POST /api/not_found HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Post);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
        assert_eq!(request_data.address, "");
        let request_data = Request::new(String::from("GET /not_found HTTP/1.1\r\n"));
        assert_eq!(request_data.syntax, RequestSyntax::Known);
        assert_eq!(request_data.method, RequestMethod::Get);
        assert_eq!(request_data.http_version, RequestHttpVersion::Http11);
        assert_eq!(request_data.address_type, RequestAddressType::Unknown);
        assert_eq!(request_data.address, "");
    }
}
