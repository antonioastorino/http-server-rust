#[derive(Debug, PartialEq)]
pub enum ContentType {
    Json,
    TextHtml,
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum ResponseStatus {
    Ok,
    NoContent,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    HttpVersionNotSupported,
}

#[derive(Debug, PartialEq)]
pub struct ResponseContents {
    pub path: &'static str,
    pub content_type: ContentType,
    pub content_size: u32,
}

impl ResponseContents {
    pub fn new_with_no_content(status: ResponseStatus) -> Self {
        Self {
            path: "",
            content_type: ContentType::Empty,
            content_size: 0,
        }
    }
}

pub struct StatusCodeAndMessage {
    pub code: u16,
    pub message: &'static str,
}

impl ContentType {
    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::TextHtml => "text/html",
            ContentType::Empty => "none",
        }
    }
}

impl ResponseStatus {
    pub fn to_code_and_message(&self) -> StatusCodeAndMessage {
        match self {
            ResponseStatus::Ok => {
                return StatusCodeAndMessage {
                    code: 200,
                    message: "OK",
                }
            }
            ResponseStatus::NoContent => {
                return StatusCodeAndMessage {
                    code: 204,
                    message: "No Content",
                }
            }
            ResponseStatus::BadRequest => {
                return StatusCodeAndMessage {
                    code: 400,
                    message: "Bad Request",
                }
            }
            ResponseStatus::NotFound => {
                return StatusCodeAndMessage {
                    code: 404,
                    message: "Not Found",
                }
            }
            ResponseStatus::MethodNotAllowed => {
                return StatusCodeAndMessage {
                    code: 405,
                    message: "Method Not Allowed",
                }
            }
            ResponseStatus::HttpVersionNotSupported => {
                return StatusCodeAndMessage {
                    code: 505,
                    message: "HTTP Version Not Supported",
                }
            }
        }
    }
}
