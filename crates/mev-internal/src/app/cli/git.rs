//! Git CLI adapter.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GitCommand {
    /// Delete a git submodule completely.
    DeleteSubmodule(crate::git::delete_submodule::DeleteSubmoduleArgs),
}

pub fn run(cmd: GitCommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        GitCommand::DeleteSubmodule(args) => {
            crate::git::run(crate::git::GitCommand::DeleteSubmodule(args))
        }
    }
}
