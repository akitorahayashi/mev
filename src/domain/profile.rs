//! Profile identifiers and mapping rules.

use std::fmt;

/// A resolved, valid provisioning profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Macbook,
    MacMini,
    Common,
}

impl Profile {
    /// Canonical string representation passed to Ansible.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Macbook => "macbook",
            Self::MacMini => "mac-mini",
            Self::Common => "common",
        }
    }

    pub fn is_machine_profile(&self) -> bool {
        matches!(self, Self::Macbook | Self::MacMini)
    }

    /// Input aliases for this profile (excluding the canonical name).
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Self::Macbook => &["mbk"],
            Self::MacMini => &["mmn"],
            Self::Common => &["cmn"],
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// All profile variants in display order.
pub fn all_profiles() -> &'static [Profile] {
    &[Profile::Common, Profile::Macbook, Profile::MacMini]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_as_str_roundtrips() {
        assert_eq!(Profile::Macbook.as_str(), "macbook");
        assert_eq!(Profile::MacMini.as_str(), "mac-mini");
        assert_eq!(Profile::Common.as_str(), "common");
    }
}
