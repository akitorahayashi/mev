//! CLI input contract for the `create` command.

use clap::Args;

use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::profile::Profile;

/// Input aliases mapping user-supplied strings to `Profile` variants.
const PROFILE_ALIASES: &[(&str, Profile)] = &[
    ("macbook", Profile::Macbook),
    ("mbk", Profile::Macbook),
    ("mac-mini", Profile::MacMini),
    ("mmn", Profile::MacMini),
    ("common", Profile::Common),
    ("cmn", Profile::Common),
];

/// Resolve a profile identifier or alias to a `Profile`.
pub(crate) fn resolve_profile(input: &str) -> Option<Profile> {
    for &(alias, profile) in PROFILE_ALIASES {
        if input == alias {
            return Some(profile);
        }
    }
    None
}

/// Validate that the input maps to a machine-specific profile (required for `create`).
fn validate_machine_profile(input: &str) -> Result<Profile, AppError> {
    let profile =
        resolve_profile(input).ok_or_else(|| AppError::InvalidProfile(input.to_string()))?;
    if !profile.is_machine_profile() {
        return Err(AppError::InvalidProfile(format!(
            "'{input}' is not a machine profile. Valid: {}",
            crate::domain::profile::all_profiles()
                .iter()
                .filter(|p| p.is_machine_profile())
                .map(Profile::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    Ok(profile)
}

#[derive(Args)]
pub struct CreateArgs {
    /// Profile to create (macbook/mbk, mac-mini/mmn).
    pub profile: String,

    /// Overwrite existing role configs with package defaults.
    #[arg(short, long)]
    pub overwrite: bool,

    /// Enable verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn run(args: CreateArgs) -> Result<(), AppError> {
    let profile = validate_machine_profile(&args.profile)?;
    api::create(profile, args.overwrite, args.verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_machine_profile_rejects_common() {
        assert!(validate_machine_profile("common").is_err());
    }

    #[test]
    fn validate_machine_profile_accepts_macbook() {
        assert_eq!(validate_machine_profile("macbook").unwrap(), Profile::Macbook);
        assert_eq!(validate_machine_profile("mbk").unwrap(), Profile::Macbook);
    }
}
