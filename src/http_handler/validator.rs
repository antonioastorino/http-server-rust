const VALID_API_ADDRESSES: [&'static str; 1] = ["/api/status"];
const VALID_ADDRESSES: [&'static str; 2] = ["/", "/index.html"];

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Unknown,
}

impl RequestMethod {
    pub fn from_str(input: &str) -> RequestMethod {
        match input {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            &_ => RequestMethod::Unknown,
        }
    }
}

pub fn validate_uri(address: &str) -> Result<(), ()> {
    if VALID_API_ADDRESSES.contains(&address) || VALID_ADDRESSES.contains(&address) {
        return Ok(());
    }
    return Err(());
}

pub fn validate_version(version: &str) -> Result<(), ()> {
    println!("{}", version);
    if version == "HTTP/1.1" {
        return Ok(());
    }
    return Err(());
}
