//! Delete a git submodule from the current repository.

use std::fs;
use std::path::{Component, Path};
use std::process::Command;

use clap::Args;

#[derive(Args)]
pub struct DeleteSubmoduleArgs {
    /// Relative path to the submodule.
    pub submodule_path: String,
}

pub fn run(args: DeleteSubmoduleArgs) -> Result<(), Box<dyn std::error::Error>> {
    delete_submodule(&args.submodule_path)
}

fn is_valid_submodule_path(path: &str) -> bool {
    let path = Path::new(path);

    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path.components().all(|component| matches!(component, Component::Normal(_)))
}

fn delete_submodule(submodule_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_submodule_path(submodule_path) {
        return Err(format!(
            "invalid submodule path '{submodule_path}': must be a relative path without traversal"
        )
        .into());
    }

    println!("Deleting submodule {submodule_path}...");

    run_required_command("git", ["submodule", "deinit", "-f", submodule_path])?;
    run_required_command("git", ["rm", "-f", "-r", submodule_path])?;

    let modules_path = Path::new(".git").join("modules").join(submodule_path);
    if modules_path.exists() {
        fs::remove_dir_all(&modules_path)?;
    }

    remove_submodule_config_section(submodule_path)?;

    println!("Submodule {submodule_path} deleted successfully.");
    Ok(())
}

fn run_required_command<const N: usize>(
    program: &str,
    args: [&str; N],
) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(program).args(args).status()?;
    if status.success() {
        return Ok(());
    }

    Err(format!("{program} {} exited with code {}", args.join(" "), status.code().unwrap_or(1))
        .into())
}

fn remove_submodule_config_section(submodule_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--remove-section", &format!("submodule.{submodule_path}")])
        .output()?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("No such section") {
        return Ok(());
    }

    Err(format!("could not remove config section: {}", stderr.trim()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_path_is_rejected() {
        assert!(!is_valid_submodule_path("/absolute/path"));
    }

    #[test]
    fn parent_traversal_is_rejected() {
        assert!(!is_valid_submodule_path("../escape/path"));
    }

    #[test]
    fn current_directory_is_rejected() {
        assert!(!is_valid_submodule_path("./vendor/some-dep"));
    }

    #[test]
    fn relative_path_is_accepted() {
        assert!(is_valid_submodule_path("vendor/some-dep"));
    }

    #[test]
    fn dotted_segment_is_accepted() {
        assert!(is_valid_submodule_path("vendor/some..dep"));
    }
}
