const CONTENT_TYPE_JSON: &'static str = "application/json";
const CONTENT_TYPE_HTML: &'static str = "text/html";
const CONTENT_TYPE_PNG: &'static str = "image/png";

pub type ContentSize = u64;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Html,
    Json,
    Png,
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
        if path.ends_with(".png") {
            return Self::Png;
        }
        return Self::Unknown;
    }

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
        return Self::Unknown;
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Json => CONTENT_TYPE_JSON,
            ContentType::Html => CONTENT_TYPE_HTML,
            ContentType::Png => CONTENT_TYPE_PNG,
            ContentType::Unknown => "",
        }
    }
}
