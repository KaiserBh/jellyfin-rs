use std::fmt;

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Debug)]
pub enum JellyfinError {
    NetworkError(reqwest::Error),
    UrlParseError(url::ParseError),
    AuthNotFound,
    HttpRequestError {
        status: u16,
        type_: Option<String>, // Using type_ because `type` is a reserved keyword in Rust
        title: Option<String>,
        detail: Option<String>,
        instance: Option<String>,
        property1: Option<String>,
        property2: Option<String>,
        message: String, // To hold a simple error message or non-JSON response body
    },
}

impl fmt::Display for JellyfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(v) => write!(f, "{}", v),
            Self::UrlParseError(v) => write!(f, "{}", v),
            Self::AuthNotFound => write!(f, "Unauthorized."),
            Self::HttpRequestError {
                status,
                type_,
                title,
                detail,
                instance,
                property1,
                property2,
                message,
            } => {
                write!(f, "HTTP Request Error (Status {}): {}", status, message)?;
                if let Some(t) = type_ {
                    write!(f, ", Type: {}", t)?;
                }
                if let Some(t) = title {
                    write!(f, ", Title: {}", t)?;
                }
                if let Some(d) = detail {
                    write!(f, ", Detail: {}", d)?;
                }
                if let Some(i) = instance {
                    write!(f, ", Instance: {}", i)?;
                }
                if let Some(p1) = property1 {
                    write!(f, ", Property1: {}", p1)?;
                }
                if let Some(p2) = property2 {
                    write!(f, ", Property2: {}", p2)?;
                }
                Ok(())
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
