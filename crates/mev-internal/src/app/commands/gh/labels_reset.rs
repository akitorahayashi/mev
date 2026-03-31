//! Delete all labels from a repository.

use clap::Args;

use crate::adapters::{gh, git};
use crate::domain::repo_target;

#[derive(Args)]
pub struct LabelsResetArgs {
    /// Target repository in [HOST/]OWNER/REPO format.
    #[arg(short = 'R', long = "repo")]
    pub repo: Option<String>,
}

pub fn run(args: LabelsResetArgs) -> Result<(), Box<dyn std::error::Error>> {
    let git_adapter = git::GitAdapter::default();
    let origin_url = args.repo.is_none().then(|| git_adapter.current_origin_url()).transpose()?;
    let repo = repo_target::resolve_repo_ref(args.repo.as_deref(), origin_url.as_deref())?;

    let gh_adapter = gh::GhAdapter::default();
    let names = gh_adapter.list_label_names(&repo)?;

    if names.is_empty() {
        println!("No labels to delete in {}.", repo.as_gh_repo_arg());
        return Ok(());
    }

    for name in names {
        println!("Deleting label {name} from {}...", repo.as_gh_repo_arg());
        gh_adapter.delete_label(&repo, &name)?;
    }

    println!("Deleted all labels from {}.", repo.as_gh_repo_arg());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serial_test::serial;
    use crate::testing::env_mock;
    use super::*;

    #[test]
    #[serial]
    #[allow(unused_unsafe)]
    fn resets_all_labels_successfully() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let git_args = temp_dir.path().join("git_args.txt");
        let gh_args = temp_dir.path().join("gh_args.txt");

        let bin_path = env_mock::create_mock_bin(
            "git",
            &temp_dir,
            &format!(r#"#!/bin/sh
                echo "$@" >> "{}"
                echo "git@github.com:owner/repo.git"
            "#, git_args.display()),
        );
        env_mock::create_mock_bin(
            "gh",
            &temp_dir,
            &format!(r#"#!/bin/sh
                echo "$@" >> "{}"
                if [ "$1" = "label" ] && [ "$2" = "list" ]; then
                    echo "bugs\nfeats"
                else
                    exit 0
                fi
            "#, gh_args.display()),
        );

        let _guard = unsafe { env_mock::PathGuard::new(&bin_path) };

        run(LabelsResetArgs { repo: None })?;

        let gh_cmds = fs::read_to_string(gh_args)?;
        assert!(gh_cmds.contains("label delete bugs"));
        assert!(gh_cmds.contains("label delete feats"));

        Ok(())
    }

    #[test]
    #[serial]
    #[allow(unused_unsafe)]
    fn skips_reset_if_no_labels() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let git_args = temp_dir.path().join("git_args.txt");
        let gh_args = temp_dir.path().join("gh_args.txt");

        let bin_path = env_mock::create_mock_bin(
            "git",
            &temp_dir,
            &format!(r#"#!/bin/sh
                echo "$@" >> "{}"
                echo "git@github.com:owner/repo.git"
            "#, git_args.display()),
        );
        env_mock::create_mock_bin(
            "gh",
            &temp_dir,
            &format!(r#"#!/bin/sh
                echo "$@" >> "{}"
                if [ "$1" = "label" ] && [ "$2" = "list" ]; then
                    echo ""
                else
                    exit 0
                fi
            "#, gh_args.display()),
        );

        let _guard = unsafe { env_mock::PathGuard::new(&bin_path) };

        run(LabelsResetArgs { repo: Some("owner/repo".to_string()) })?;

        let gh_cmds = fs::read_to_string(gh_args)?;
        assert!(!gh_cmds.contains("label delete"));

        Ok(())
    }
}
