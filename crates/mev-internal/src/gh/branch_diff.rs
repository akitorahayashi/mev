//! Show a git diff with lightweight repository context.

use std::path::Path;
use std::process::Command;

use clap::Args;

#[derive(Args)]
pub struct BranchDiffArgs {
    /// Base branch for the diff range.
    #[arg(default_value = "main")]
    pub base_branch: String,

    /// Target branch for the diff range.
    #[arg(default_value = "HEAD")]
    pub target_branch: String,
}

pub fn run(args: BranchDiffArgs) -> Result<(), Box<dyn std::error::Error>> {
    let repo_name = current_repo_name()?;

    println!("=== DIFF CONTEXT ===");
    println!("Base: {}", args.base_branch);
    println!("Target: {}", args.target_branch);
    println!("Repository: {repo_name}");
    println!("=== CHANGES ===");

    let diff_range = format!("{}...{}", args.base_branch, args.target_branch);
    let status = Command::new("git").args(["diff", &diff_range]).status()?;
    if status.success() {
        return Ok(());
    }

    Err(format!("git diff {diff_range} exited with code {}", status.code().unwrap_or(1)).into())
}

fn current_repo_name() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?;

    if !output.status.success() {
        return Err(format!(
            "git rev-parse --show-toplevel exited with code {}",
            output.status.code().unwrap_or(1)
        )
        .into());
    }

    let repo_root = String::from_utf8(output.stdout)?;
    let repo_name =
        Path::new(repo_root.trim()).file_name().ok_or("could not determine repository name")?;
    Ok(repo_name.to_string_lossy().into_owned())
}
