//! CLI input contract for the `backup` command.

use clap::Args;

use crate::app::api;
use crate::domain::error::AppError;

#[derive(Args)]
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
    } else if let Some(target) = args.target {
        api::backup(target.as_str())
    } else {
        Err(AppError::Backup("Target is required unless --list is used.".to_string()))
    }
}
