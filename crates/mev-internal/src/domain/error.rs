//! Internal domain errors.

use thiserror::Error;

/// Core domain error for the `mev-internal` execution context.
#[derive(Debug, Error)]
pub enum InternalError {
    /// Failed to parse a domain object.
    #[error("failed to parse: {0}")]
    Parse(String),

    /// Process execution failed.
    #[error("process failed: {message}")]
    Process {
        /// The error message.
        message: String,
        /// Optional exit code if the process exited.
        exit_code: Option<i32>,
    },

    /// An error related to configuration.
    #[error("configuration error: {0}")]
    Config(String),

    /// A JSON serialization or deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Missing requirement.
    #[error("missing requirement: {0}")]
    MissingRequirement(String),

    /// Generic validation error.
    #[error("validation error: {0}")]
    Validation(String),
}

impl From<std::io::Error> for InternalError {
    fn from(err: std::io::Error) -> Self {
        Self::Process { message: format!("io error: {err}"), exit_code: None }
    }
}

impl From<std::string::FromUtf8Error> for InternalError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Parse(format!("invalid utf8: {err}"))
    }
}
