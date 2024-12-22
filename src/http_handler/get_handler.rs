use crate::http_response_status::*;

const VALID_API_ADDRESSES: [&'static str; 1] = ["/api/status"];
const VALID_ADDRESSES: [&'static str; 2] = ["/", "/index.html"];
static STATUS_PATH: &'static str = "assets/status.json";

pub fn validate(address: &str) -> ResponseStatus {
    if VALID_API_ADDRESSES.contains(&address) {
        return ResponseStatus::Ok((STATUS_PATH, ContentType::Json));
    }

    if address == VALID_ADDRESSES[0] {
        return ResponseStatus::Ok(("path/to/index.html", ContentType::TextHtml));
    } else if address == VALID_ADDRESSES[1] {
        return ResponseStatus::Ok(("path/to/index.html", ContentType::TextHtml));
    }

    return ResponseStatus::NotFound(("/path/to/not_found.html", ContentType::TextHtml));
}
