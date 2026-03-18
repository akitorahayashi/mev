//! CLI input contract for the `backup` command.

use clap::Args;

use crate::app::api;
use crate::domain::backup_target::BackupTarget;
use crate::domain::error::AppError;

/// Resolve a user input string to a backup target.
fn resolve_backup_target(s: &str) -> Option<BackupTarget> {
    match s.to_lowercase().as_str() {
        "system" => Some(BackupTarget::System),
        "vscode" | "vscode-extensions" => Some(BackupTarget::Vscode),
        _ => None,
    }
}

#[derive(Args)]
#[command(group(
    clap::ArgGroup::new("action")
        .required(true)
        .args(["list", "target"]),
))]
pub struct BackupArgs {
    #[arg(short = 'l', long = "list", aliases = ["ls"], action = clap::ArgAction::SetTrue, help = "List available backup targets")]
    pub list: bool,

    /// Backup target (system, vscode).
    pub target: Option<String>,
}

pub fn run(args: BackupArgs) -> Result<(), AppError> {
    if args.list {
        api::backup_list();
        Ok(())
    } else if let Some(target_input) = args.target {
        let target = resolve_backup_target(&target_input).ok_or_else(|| {
            let valid: Vec<_> = BackupTarget::all().iter().map(|t| t.name()).collect();
            AppError::Backup(format!(
                "unknown backup target '{}'. Valid targets: {}",
                target_input,
                valid.join(", ")
            ))
        })?;
        api::backup(target)
    } else {
        // Controlled by ArgGroup(required=true)
        unreachable!("clap ensures either list or target is present")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_system() {
        assert_eq!(resolve_backup_target("system"), Some(BackupTarget::System));
        assert_eq!(resolve_backup_target("System"), Some(BackupTarget::System));
    }

    #[test]
    fn resolves_vscode() {
        assert_eq!(resolve_backup_target("vscode"), Some(BackupTarget::Vscode));
    }

    #[test]
    fn resolves_vscode_extensions_alias() {
        assert_eq!(resolve_backup_target("vscode-extensions"), Some(BackupTarget::Vscode));
    }

    #[test]
    fn rejects_unknown() {
        assert_eq!(resolve_backup_target("unknown"), None);
    }
}
