const CONTENT_TYPE_HTML: &'static str = "text/html";
const CONTENT_TYPE_JSON: &'static str = "application/json";
const CONTENT_TYPE_PNG: &'static str = "image/png";
const CONTENT_TYPE_JPEG: &'static str = "image/jpeg";
const CONTENT_TYPE_CSS: &'static str = "text/css";
const CONTENT_TYPE_JAVASCRIPT: &'static str = "text/javascript";
const CONTENT_TYPE_TEXT: &'static str = "text/plain";

pub type ContentLength = u64;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Html,
    Json,
    Png,
    Jpeg,
    Css,
    Javascript,
    Text,
    Unknown,
}

impl ContentType {
    // The server's payload is mapped to a specific variant using the file extension
    pub fn from_file_name(path_str: &str) -> Self {
        let path = path_str.to_uppercase();
        if path.ends_with(".JSON") {
            return Self::Json;
        }
        if path.ends_with(".HTML") {
            return Self::Html;
        }
        if path.ends_with(".PNG") {
            return Self::Png;
        }
        if path.ends_with(".JPEG") || path.ends_with(".JPG") {
            return Self::Jpeg;
        }
        if path.ends_with(".CSS") {
            return Self::Css;
        }
        if path.ends_with(".JS") {
            return Self::Javascript;
        }
        if path.ends_with(".TXT") {
            return Self::Text;
        }
        return Self::Unknown;
    }

    // The client's 'Content-Type' value is mapped to a specific variant
    pub fn from_content_type_str(content_type_str: &str) -> Self {
        if content_type_str == CONTENT_TYPE_JSON {
            return Self::Json;
        }
        if content_type_str == CONTENT_TYPE_HTML {
            return Self::Html;
        }
        if content_type_str == CONTENT_TYPE_PNG {
            return Self::Png;
        }
        if content_type_str == CONTENT_TYPE_JPEG {
            return Self::Jpeg;
        }
        if content_type_str == CONTENT_TYPE_CSS {
            return Self::Css;
        }
        if content_type_str == CONTENT_TYPE_JAVASCRIPT {
            return Self::Javascript;
        }
        if content_type_str == CONTENT_TYPE_TEXT {
            return Self::Text;
        }
        return Self::Unknown;
    }

    pub fn is_binary(&self) -> bool {
        match self {
            ContentType::Png | ContentType::Jpeg => true,
            _ => false,
        }
    }

    // A specific variant is mapped to a 'Content-Type' value
    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Json => CONTENT_TYPE_JSON,
            ContentType::Html => CONTENT_TYPE_HTML,
            ContentType::Png => CONTENT_TYPE_PNG,
            ContentType::Jpeg => CONTENT_TYPE_JPEG,
            ContentType::Css => CONTENT_TYPE_CSS,
            ContentType::Javascript => CONTENT_TYPE_JAVASCRIPT,
            ContentType::Text => CONTENT_TYPE_TEXT,
            ContentType::Unknown => "",
        }
    }
}
