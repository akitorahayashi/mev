//! Domain-level typed errors for mev-internal.

use std::fmt;

/// Domain errors representing rules and constraints violations.
#[derive(Debug, Eq, PartialEq)]
pub enum DomainError {
    /// The provided repository reference string is invalid.
    InvalidRepositoryRef(String),

    /// The remote URL format is unsupported.
    UnsupportedRemoteUrl(String),

    /// The origin URL is missing and no explicit repository was provided.
    MissingOriginUrl(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRepositoryRef(msg) => write!(f, "invalid repository reference: {msg}"),
            Self::UnsupportedRemoteUrl(msg) => write!(f, "unsupported remote url: {msg}"),
            Self::MissingOriginUrl(msg) => write!(f, "missing origin url: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}
