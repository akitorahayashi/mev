//! VCS identity model and switch identity resolution.
//!
//! `VcsIdentity` is a mev-specific concept: it represents the name/email pair
//! stored per identity (personal / work) and applied to Git and Jujutsu.

use std::fmt;

/// Name and email pair applied to global VCS configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VcsIdentity {
    pub name: String,
    pub email: String,
}

/// A resolved, valid VCS identity target for switching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchIdentity {
    Personal,
    Work,
}

impl SwitchIdentity {
    /// Canonical string representation used for storage lookups and display.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Work => "work",
        }
    }
}

impl fmt::Display for SwitchIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_identity_as_str_roundtrips() {
        assert_eq!(SwitchIdentity::Personal.as_str(), "personal");
        assert_eq!(SwitchIdentity::Work.as_str(), "work");
    }
}
