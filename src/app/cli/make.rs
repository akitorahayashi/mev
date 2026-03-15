//! CLI input contract for the `make` command.

use clap::Args;

use crate::app::api;
use crate::app::cli::create::resolve_profile;
use crate::domain::error::AppError;
use crate::domain::profile::Profile;

/// Validate any profile including `common` (required for `make`).
fn validate_profile(input: &str) -> Result<Profile, AppError> {
    resolve_profile(input).ok_or_else(|| AppError::InvalidProfile(input.to_string()))
}

#[derive(Args)]
pub struct MakeArgs {
    /// Ansible tag to run (e.g., rust, python-tools, shell, brew-cask).
    pub tag: String,

    /// Profile to use (common, macbook/mbk, mac-mini/mmn).
    #[arg(short = 'p', long, default_value = "common")]
    pub profile: String,

    /// Overwrite existing role configs with package defaults.
    #[arg(short, long)]
    pub overwrite: bool,

    /// Enable verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn run(args: MakeArgs) -> Result<(), AppError> {
    let profile = validate_profile(&args.profile)?;
    api::make(profile, &args.tag, args.overwrite, args.verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_canonical_profiles() {
        assert_eq!(resolve_profile("common"), Some(Profile::Common));
        assert_eq!(resolve_profile("macbook"), Some(Profile::Macbook));
        assert_eq!(resolve_profile("mac-mini"), Some(Profile::MacMini));
    }

    #[test]
    fn resolves_aliases() {
        assert_eq!(resolve_profile("mbk"), Some(Profile::Macbook));
        assert_eq!(resolve_profile("mmn"), Some(Profile::MacMini));
        assert_eq!(resolve_profile("cmn"), Some(Profile::Common));
    }

    #[test]
    fn rejects_unknown() {
        assert_eq!(resolve_profile("desktop"), None);
    }
}
