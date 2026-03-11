//! Print pull request comments.

use std::process::Command;

use clap::Args;

#[derive(Args)]
pub struct PrCommentsArgs {
    /// Pull request number.
    pub pr_id: u64,
}

pub fn run(args: PrCommentsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("gh")
        .args([
            "pr",
            "view",
            &args.pr_id.to_string(),
            "--json",
            "comments",
            "--jq",
            r#".comments[] | "[\(.author.login)] \(.body)""#,
        ])
        .status()?;

    if status.success() {
        return Ok(());
    }

    Err(format!(
        "gh pr view {} comments exited with code {}",
        args.pr_id,
        status.code().unwrap_or(1)
    )
    .into())
}
