use super::common::*;
use crate::http_handler::request::{
    RequestHeader, RequestHttpVersion, RequestMethod, RequestSyntax,
};

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
    pub content_length: u64,
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: ResponseStatus,
    pub payload: ResponsePayload,
}

impl Response {
    pub fn new(request_data: &RequestHeader) -> Self {
        let mut content_length = 0;
        let (status, path) = if request_data.syntax == RequestSyntax::Unknown {
            (ResponseStatus::BadRequest, "www/bad_request.html")
        } else if request_data.method == RequestMethod::Unknown {
            (
                ResponseStatus::MethodNotAllowed,
                "www/method_not_allowed.html",
            )
        } else if request_data.http_version == RequestHttpVersion::Unknown {
            (
                ResponseStatus::HttpVersionNotSupported,
                "www/http_version_not_supported.html",
            )
        } else {
            // The syntax is ok. Post method don't have content -> successful request
            if request_data.method == RequestMethod::Post {
                (ResponseStatus::NoContent, "")
            } else if request_data.address == "" {
                (ResponseStatus::NotFound, "www/not_found.html")
            } else {
                // Get requests want a file -> check that the file exists
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
        if status != ResponseStatus::NoContent {
            content_length = std::fs::metadata(path).unwrap().len();
        }
        return Self {
            status,
            payload: ResponsePayload {
                path,
                content_type: ContentType::from_file_name(path),
                content_length,
            },
        };
    }
}
