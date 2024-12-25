use crate::http_handler::request::*;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Json,
    Html,
    Unknown,
}

impl ContentType {
    pub fn from_file_name(path: &str) -> Self {
        if path.ends_with(".json") {
            return Self::Json;
        }
        if path.ends_with(".html") {
            return Self::Html;
        }
        return Self::Unknown;
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Html => "text/html",
            ContentType::Unknown => "",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResponseStatus {
    Ok,
    NoContent,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    HttpVersionNotSupported,
    InternalServerError,
}

pub struct StatusCodeAndMessage {
    pub code: u16,
    pub message: &'static str,
}

impl ResponseStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            ResponseStatus::Ok => {
                return "200 OK";
            }
            ResponseStatus::NoContent => {
                return "204 No Content";
            }
            ResponseStatus::BadRequest => {
                return "400 Bad Request";
            }
            ResponseStatus::NotFound => {
                return "404 Not Found";
            }
            ResponseStatus::MethodNotAllowed => {
                return "405 Method Not Allowed";
            }
            ResponseStatus::InternalServerError => {
                return "500 Internal Server Error";
            }
            ResponseStatus::HttpVersionNotSupported => {
                return "505 HTTP Version Not Supported";
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ResponsePayload {
    pub path: &'static str,
    pub content_type: ContentType,
    pub content_size: u64,
}
#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: ResponseStatus,
    pub payload: ResponsePayload,
}

impl Response {
    pub fn new(request_data: &Request) -> Self {
        let mut content_size = 0;
        let mut content_type = ContentType::Unknown;
        let (status, path) = if request_data.syntax == RequestSyntax::Unknown {
            (ResponseStatus::BadRequest, "www/bad_request.html")
        } else if request_data.http_version == RequestHttpVersion::Unknown {
            (
                ResponseStatus::HttpVersionNotSupported,
                "www/http_version_not_supported.html",
            )
        } else if request_data.method == RequestMethod::Unknown {
            (
                ResponseStatus::MethodNotAllowed,
                "www/method_not_allowed.html",
            )
        } else if request_data.address_type == RequestAddressType::Unknown {
            (ResponseStatus::NotFound, "www/not_found.html")
        } else {
            if request_data.method == RequestMethod::Post {
                (ResponseStatus::NoContent, request_data.address)
            } else {
                // verify that the file exists
                if std::path::Path::new(request_data.address).exists() {
                    (ResponseStatus::Ok, request_data.address)
                } else {
                    (
                        ResponseStatus::InternalServerError,
                        "www/internal_server_error.html",
                    )
                }
            }
        };
        if request_data.method != RequestMethod::Post {
            content_size = std::fs::metadata(path).unwrap().len();
        }
        return Self {
            status,
            payload: ResponsePayload {
                path,
                content_type: ContentType::from_file_name(path),
                content_size,
            },
        };
    }
}
