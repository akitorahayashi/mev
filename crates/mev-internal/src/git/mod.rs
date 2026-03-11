//! Git command implementations.

pub mod delete_submodule;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GitCommand {
    /// Delete a git submodule completely.
    DeleteSubmodule(delete_submodule::DeleteSubmoduleArgs),
}

pub fn run(cmd: GitCommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        GitCommand::DeleteSubmodule(args) => delete_submodule::run(args),
    }
}
