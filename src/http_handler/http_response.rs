use crate::http_response_status::*;

pub fn http_response_header_create(_status: &ResponseStatus) -> String {
    return String::from("hello");
}
