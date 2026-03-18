//! CLI input contract for the `switch` command.

use clap::Args;

use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::vcs_identity::SwitchIdentity;

/// Input aliases mapping user-supplied strings to `SwitchIdentity` variants.
const SWITCH_IDENTITY_ALIASES: &[(&str, SwitchIdentity)] = &[
    ("p", SwitchIdentity::Personal),
    ("personal", SwitchIdentity::Personal),
    ("w", SwitchIdentity::Work),
    ("work", SwitchIdentity::Work),
];

/// Resolve a switch identity input (alias or canonical) to a `SwitchIdentity`.
fn resolve_switch_identity(input: &str) -> Option<SwitchIdentity> {
    let lower = input.to_lowercase();
    SWITCH_IDENTITY_ALIASES
        .iter()
        .find(|&&(alias, _)| alias == lower.as_str())
        .map(|&(_, identity)| identity)
}

#[derive(Args)]
pub struct SwitchArgs {
    /// Identity to switch to (personal/p, work/w).
    pub identity: String,
}

pub fn run(args: SwitchArgs) -> Result<(), AppError> {
    let identity = resolve_switch_identity(&args.identity).ok_or_else(|| {
        AppError::InvalidIdentity(format!(
            "invalid identity '{}'. Valid: personal (p), work (w)",
            args.identity
        ))
    })?;
    api::switch(identity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_switch_identities() {
        assert_eq!(resolve_switch_identity("p"), Some(SwitchIdentity::Personal));
        assert_eq!(resolve_switch_identity("personal"), Some(SwitchIdentity::Personal));
        assert_eq!(resolve_switch_identity("w"), Some(SwitchIdentity::Work));
        assert_eq!(resolve_switch_identity("work"), Some(SwitchIdentity::Work));
        assert_eq!(resolve_switch_identity("unknown"), None);
    }
}
