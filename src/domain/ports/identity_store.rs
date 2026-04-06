//! Configuration storage port.

use std::path::Path;

use crate::domain::error::AppError;
use crate::domain::identity::{Identity, IdentityConfig, IdentityScope};

/// Persists and retrieves Git identity configuration.
pub trait IdentityStore {
    /// Check if identity configuration file exists.
    fn exists(&self) -> bool;

    /// Load the full identity configuration.
    fn load(&self) -> Result<IdentityConfig, AppError>;

    /// Save the full identity configuration.
    fn save(&self, config: &IdentityConfig) -> Result<(), AppError>;

    /// Get the identity for the given switch target.
    fn get_identity(&self, identity: IdentityScope) -> Result<Option<Identity>, AppError>;

    /// Get the identity configuration file path.
    fn identity_path(&self) -> &Path;
}
