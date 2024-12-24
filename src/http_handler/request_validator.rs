const VALID_API_ADDRESSES_GET: [&'static str; 1] = ["/api/status"];
const VALID_API_ADDRESSES_POST: [&'static str; 1] = ["/api/set"];
const VALID_ADDRESSES_GET: [&'static str; 2] = ["/", "/index.html"];

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
}

#[derive(Debug, PartialEq)]
pub enum RequestAddressType {
    Uri,
    Url,
}

#[derive(Debug, PartialEq)]
pub struct RequestData {
    pub method: RequestMethod,
    pub address_type: RequestAddressType,
}

pub fn validate_method(input: &str) -> Result<RequestMethod, ()> {
    match input {
        "GET" => Ok(RequestMethod::Get),
        "POST" => Ok(RequestMethod::Post),
        _ => return Err(()),
    }
}

pub fn validate_address(address: &str, method: &RequestMethod) -> Result<RequestAddressType, ()> {
    match method {
        RequestMethod::Get => {
            if VALID_API_ADDRESSES_GET.contains(&address) {
                return Ok(RequestAddressType::Uri);
            }
            if VALID_ADDRESSES_GET.contains(&address) {
                return Ok(RequestAddressType::Url);
            }
            return Err(());
        }
        RequestMethod::Post => {
            if VALID_API_ADDRESSES_POST.contains(&address) {
                return Ok(RequestAddressType::Url);
            }
            return Err(());
        }
    }
}

pub fn validate_version(version: &str) -> Result<(), ()> {
    println!("{}", version);
    if version == "HTTP/1.1" {
        return Ok(());
    }
    return Err(());
}
