use std::fmt;

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Debug)]
pub enum JellyfinError {
    NetworkError(reqwest::Error),
    UrlParseError(url::ParseError),
    AuthNotFound,
    HttpRequestError { status: u16, message: String },
}

impl fmt::Display for JellyfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(v) => write!(f, "{}", v),
            Self::UrlParseError(v) => write!(f, "{}", v),
            Self::AuthNotFound => write!(f, "Unauthorized."),
            Self::HttpRequestError { status, message } => {
                write!(f, "HTTP Request Error (Status {}): {}", status, message)
            }
        }
    }
}

impl std::error::Error for JellyfinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NetworkError(e) => Some(e),
            Self::UrlParseError(e) => Some(e),
            // AuthNotFound does not wrap another error, so we return None
            Self::AuthNotFound | Self::HttpRequestError { .. } => None,
        }
    }
}

impl From<reqwest::Error> for JellyfinError {
    fn from(value: reqwest::Error) -> Self {
        Self::NetworkError(value)
    }
}

impl From<url::ParseError> for JellyfinError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}
