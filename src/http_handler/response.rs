use crate::http_handler::request_validator::RequestData;
use crate::http_response_status::*;

pub fn from_uri_to_path(path_str: &str) -> &'static str {
    if ["/", "/index.html"].contains(&path_str) {
        return "www/index.html";
    }
    return "www/not_found.html";
}

#[derive(Debug, PartialEq)]
pub struct ResponsePayload {
    pub path: &'static str,
    pub content_type: ContentType,
    pub content_size: u32,
}
#[derive(Debug, PartialEq)]
pub struct Response {
    pub header: String,
    pub payload: ResponsePayload,
}

impl Response {
    pub fn new(request_data: &RequestData) -> Self {
        todo!()
    }
}
