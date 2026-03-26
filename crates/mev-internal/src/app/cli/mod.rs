//! CLI adapter — top-level parser and subcommand dispatch.

pub mod gh;
pub mod git;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mev-internal")]
#[command(version)]
#[command(about = "Internal command runtime for mev")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Git operations.
    #[command(subcommand)]
    Git(git::GitCommand),

    /// GitHub CLI operations.
    #[command(subcommand)]
    Gh(gh::GhCommand),
}

/// Entry point for the CLI.
pub fn run() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Git(cmd) => git::run(cmd),
        Commands::Gh(cmd) => gh::run(cmd),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_shape_gh_labels_deploy() {
        let err = Cli::command().try_get_matches_from(["mev-internal", "gh", "labels", "deploy", "--help"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn verify_cli_shape_gh_labels_reset() {
        let err = Cli::command().try_get_matches_from(["mev-internal", "gh", "labels", "reset", "--help"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn verify_cli_shape_git_delete_submodule() {
        let err = Cli::command().try_get_matches_from(["mev-internal", "git", "delete-submodule", "--help"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }
}
