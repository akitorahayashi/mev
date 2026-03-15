//! CLI input contract for the `backup` command.

use clap::Args;

use crate::app::api;
use crate::domain::error::AppError;

#[derive(Args)]
pub struct BackupArgs {
    /// Backup target (system, vscode, or 'list' to show available targets).
    pub target: String,
}

pub fn run(args: BackupArgs) -> Result<(), AppError> {
    if args.target == "list" {
        api::backup_list();
        Ok(())
    } else {
        api::backup(args.target.as_str())
    }
}
