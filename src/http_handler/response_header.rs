use crate::http_response_status::*;

pub fn from_uri_to_path(path_str: &str) -> &'static str {
    if ["/", "/index.html"].contains(&path_str) {
        return "www/index.html";
    }
    return "www/not_found.html";
}

pub fn create(status: &ResponseStatus) -> String {
    let status_struct = status.to_code_and_message();
    let mut ret_header =
        String::from(format!("HTTP/1.1 {} {}", status_struct.code, status_struct.message).as_str());
    match status {
        _ => {
            ret_header.push_str("\r\n\r\n");
        }
    };
    return ret_header;
}
