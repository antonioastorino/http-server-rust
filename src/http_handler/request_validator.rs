const VALID_API_ADDRESSES_GET: [&'static str; 1] = ["/api/status"];
const VALID_API_ADDRESSES_POST: [&'static str; 1] = ["/api/set"];
const VALID_ADDRESSES_GET: [&'static str; 2] = ["/", "/index.html"];

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
pub struct RequestData {
    pub syntax: RequestSyntax,
    pub http_version: RequestHttpVersion,
    pub method: RequestMethod,
    pub address_type: RequestAddressType,
}

impl RequestData {
    pub fn new() -> Self {
        Self {
            syntax: RequestSyntax::Unknown,
            http_version: RequestHttpVersion::Unknown,
            method: RequestMethod::Unknown,
            address_type: RequestAddressType::Unknown,
        }
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
            if VALID_API_ADDRESSES_GET.contains(&address) {
                return RequestAddressType::Uri;
            }
            if VALID_ADDRESSES_GET.contains(&address) {
                return RequestAddressType::Url;
            }
            return RequestAddressType::Unknown;
        }
        RequestMethod::Post => {
            if VALID_API_ADDRESSES_POST.contains(&address) {
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
    println!("{}", version);
    if version == "HTTP/1.1" {
        return RequestHttpVersion::Http11;
    }
    return RequestHttpVersion::Unknown;
}
