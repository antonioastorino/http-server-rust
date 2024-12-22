use crate::http_response_status::ResponseStatus;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    Get,
    Post,
    Unknown,
}

impl RequestType {
    pub fn from_str(input: &str) -> RequestType {
        match input {
            "GET" => RequestType::Get,
            "POST" => RequestType::Post,
            &_ => RequestType::Unknown,
        }
    }
}

pub fn verify_version(version: &str) -> Result<(), ()> {
    println!("{}", version);
    if version == "HTTP/1.1" {
        return Ok(());
    }
    return Err(());
}
