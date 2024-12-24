use crate::http_response_status::*;

pub fn create(status: &ResponseStatus) -> String {
    let status_struct = status.to_code_and_message();
    return String::from(format!("{} {}", status_struct.code, status_struct.message));
}
