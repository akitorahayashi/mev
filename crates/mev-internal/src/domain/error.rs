//! Domain-level typed errors for internal commands.

use std::fmt;

/// Top-level internal domain error.
#[derive(Debug)]
pub enum DomainError {
    /// Validation error.
    Validation(String),

    /// Process execution failed.
    ProcessExecution(String),

    /// I/O error.
    Io(std::io::Error),

    /// JSON parsing error.
    Json(serde_json::Error),

    /// UTF-8 parsing error.
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(msg) => write!(f, "{msg}"),
            Self::ProcessExecution(msg) => write!(f, "{msg}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
            Self::Utf8(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for DomainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Json(err) => Some(err),
            Self::Utf8(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::string::FromUtf8Error> for DomainError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<String> for DomainError {
    fn from(msg: String) -> Self {
        Self::Validation(msg)
    }
}

impl From<&str> for DomainError {
    fn from(msg: &str) -> Self {
        Self::Validation(msg.to_string())
    }
}
