use std::fmt;

#[derive(Debug)]
pub enum TidalError {
    Api { status: u16, message: String },
    Auth(String),
    Network(reqwest::Error),
    Json(serde_json::Error),
    Decode(String),
    Encryption(String),
    Manifest(String),
    Xml(String),
    Io(std::io::Error),
}

impl fmt::Display for TidalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TidalError::Api { status, message } => write!(f, "API error {}: {}", status, message),
            TidalError::Auth(msg) => write!(f, "Authentication failed: {}", msg),
            TidalError::Network(e) => write!(f, "Network error: {}", e),
            TidalError::Json(e) => write!(f, "JSON error: {}", e),
            TidalError::Decode(msg) => write!(f, "Decode error: {}", msg),
            TidalError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            TidalError::Manifest(msg) => write!(f, "Manifest error: {}", msg),
            TidalError::Xml(msg) => write!(f, "XML parse error: {}", msg),
            TidalError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for TidalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TidalError::Network(e) => Some(e),
            TidalError::Json(e) => Some(e),
            TidalError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for TidalError {
    fn from(e: reqwest::Error) -> Self {
        TidalError::Network(e)
    }
}

impl From<serde_json::Error> for TidalError {
    fn from(e: serde_json::Error) -> Self {
        TidalError::Json(e)
    }
}

impl From<std::io::Error> for TidalError {
    fn from(e: std::io::Error) -> Self {
        TidalError::Io(e)
    }
}

impl From<base64::DecodeError> for TidalError {
    fn from(e: base64::DecodeError) -> Self {
        TidalError::Decode(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for TidalError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        TidalError::Decode(e.to_string())
    }
}

impl From<std::array::TryFromSliceError> for TidalError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        TidalError::Decode(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, TidalError>;
