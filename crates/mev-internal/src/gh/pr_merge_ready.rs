//! Merge a pull request when GitHub reports it as mergeable.

use std::process::Command;

use clap::Args;

#[derive(Args)]
pub struct PrMergeReadyArgs {
    /// Pull request number.
    pub pr_id: u64,
}

pub fn run(args: PrMergeReadyArgs) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("gh")
        .args(["pr", "view", &args.pr_id.to_string(), "--json", "mergeable", "--jq", ".mergeable"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "gh pr view {} mergeable exited with code {}",
            args.pr_id,
            output.status.code().unwrap_or(1)
        )
        .into());
    }

    let mergeable = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if mergeable != "MERGEABLE" {
        return Err(format!("PR #{} is not mergeable: {mergeable}", args.pr_id).into());
    }

    println!("PR #{} is MERGEABLE. Merging...", args.pr_id);
    let status = Command::new("gh").args(["pr", "merge", &args.pr_id.to_string()]).status()?;

    if status.success() {
        return Ok(());
    }

    Err(format!("gh pr merge {} exited with code {}", args.pr_id, status.code().unwrap_or(1))
        .into())
}
