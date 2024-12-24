use crate::http_response_status::*;

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
