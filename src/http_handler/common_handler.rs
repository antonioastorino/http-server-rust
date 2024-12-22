#[derive(Debug, PartialEq)]
pub enum RequestType {
    GET,
    POST,
    UNKNOWN,
}

impl RequestType {
    pub fn from_str(input: &str) -> RequestType {
        match input {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            &_ => RequestType::UNKNOWN,
        }
    }
}
