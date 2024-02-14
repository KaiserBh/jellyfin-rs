use std::{collections::HashMap, fmt};

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Debug)]
pub enum JellyfinError {
    NetworkError(reqwest::Error),
    UrlParseError(url::ParseError),
    AuthNotFound,
    HttpRequestError {
        status: u16,
        type_: Option<String>,
        title: Option<String>,
        detail: Option<String>,
        instance: Option<String>,
        errors: HashMap<String, Vec<String>>, // Dynamic error fields
        message: String,
    },
}

impl fmt::Display for JellyfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "{}", e),
            Self::UrlParseError(e) => write!(f, "{}", e),
            Self::AuthNotFound => write!(f, "Unauthorized"),
            Self::HttpRequestError {
                status,
                type_,
                title,
                detail,
                instance,
                errors,
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
                for (key, messages) in errors {
                    write!(f, ", {}: {:?}", key, messages)?;
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
