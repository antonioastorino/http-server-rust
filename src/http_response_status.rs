#[derive(Debug, PartialEq)]
pub enum ContentType {
    Json,
    TextHtml,
}

pub type ResponseContent = (&'static str, ContentType);
pub type StatusCodeAndMessage = (
    u16,                     /* code */
    &'static str,            /* repr */
    Option<ResponseContent>, /*type and path */
);

impl ContentType {
    pub fn to_str(self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::TextHtml => "text/html",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResponseStatus {
    Ok(ResponseContent),
    NoContent,
    BadRequest,
    NotFound(ResponseContent),
}

impl ResponseStatus {
    pub fn to_code_and_message(self) -> StatusCodeAndMessage {
        match self {
            ResponseStatus::Ok(response_content) => return (200, "Ok", Some(response_content)),
            ResponseStatus::NoContent => return (204, "No Content", None),
            ResponseStatus::BadRequest => return (400, "Bad Request", None),
            ResponseStatus::NotFound(response_content) => return (404, "Not Found", Some(response_content)),
        }
    }
}
