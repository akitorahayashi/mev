//! GitHub CLI adapter.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GhCommand {
    /// Show the diff between two branches.
    BranchDiff(crate::gh::branch_diff::BranchDiffArgs),

    /// Print pull request comments.
    PrComments(crate::gh::pr_comments::PrCommentsArgs),

    /// Merge a pull request only when GitHub reports it as mergeable.
    PrMergeReady(crate::gh::pr_merge_ready::PrMergeReadyArgs),
}

pub fn run(cmd: GhCommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        GhCommand::BranchDiff(args) => crate::gh::run(crate::gh::GhCommand::BranchDiff(args)),
        GhCommand::PrComments(args) => crate::gh::run(crate::gh::GhCommand::PrComments(args)),
        GhCommand::PrMergeReady(args) => crate::gh::run(crate::gh::GhCommand::PrMergeReady(args)),
    }
}
