//! GitHub CLI command implementations.

pub mod branch_diff;
pub mod pr_comments;
pub mod pr_merge_ready;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GhCommand {
    /// Show the diff between two branches.
    BranchDiff(branch_diff::BranchDiffArgs),

    /// Print pull request comments.
    PrComments(pr_comments::PrCommentsArgs),

    /// Merge a pull request only when GitHub reports it as mergeable.
    PrMergeReady(pr_merge_ready::PrMergeReadyArgs),
}

pub fn run(cmd: GhCommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        GhCommand::BranchDiff(args) => branch_diff::run(args),
        GhCommand::PrComments(args) => pr_comments::run(args),
        GhCommand::PrMergeReady(args) => pr_merge_ready::run(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subcommand_names_are_kebab_case() {
        let command = GhCommand::augment_subcommands(clap::Command::new("gh"));
        let names = command
            .get_subcommands()
            .map(|subcommand| subcommand.get_name().to_owned())
            .collect::<Vec<_>>();

        assert!(names.iter().any(|name| name == "branch-diff"));
        assert!(names.iter().any(|name| name == "pr-comments"));
        assert!(names.iter().any(|name| name == "pr-merge-ready"));
    }
}
