//! Deploy the bundled label catalog to a repository.

use clap::Args;

use crate::adapters::{gh, git};
use crate::domain::{label_catalog, repo_target};

#[derive(Args)]
pub struct LabelsDeployArgs {
    /// Target repository in [HOST/]OWNER/REPO format.
    #[arg(short = 'R', long = "repo")]
    pub repo: Option<String>,
}

pub fn run(args: LabelsDeployArgs) -> Result<(), Box<dyn std::error::Error>> {
    let git_adapter = git::GitAdapter::default();
    let origin_url = args.repo.is_none().then(|| git_adapter.current_origin_url()).transpose()?;
    let repo = repo_target::resolve_repo_ref(args.repo.as_deref(), origin_url.as_deref())?;

    let gh_adapter = gh::GhAdapter::default();
    let existing_names = gh_adapter.list_label_names(&repo)?;
    let label_specs = label_catalog::load_bundled_labels()?;

    for spec in label_specs {
        if existing_names.iter().any(|name| name == &spec.name) {
            println!("Replacing label {} in {}...", spec.name, repo.as_gh_repo_arg());
            gh_adapter.delete_label(&repo, &spec.name)?;
        } else {
            println!("Creating label {} in {}...", spec.name, repo.as_gh_repo_arg());
        }

        gh_adapter.create_label(&repo, &spec)?;
    }

    println!("Deployed bundled labels to {}.", repo.as_gh_repo_arg());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::env_mock;
    use serial_test::serial;
    use std::fs;

    #[test]
    #[serial]
    #[allow(unused_unsafe)]
    fn deploys_labels_successfully_without_replacements() -> Result<(), Box<dyn std::error::Error>>
    {
        let temp_dir = tempfile::tempdir()?;
        let git_args = temp_dir.path().join("git_args.txt");
        let gh_args = temp_dir.path().join("gh_args.txt");

        let bin_path = env_mock::create_mock_bin(
            "git",
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" >> "{}"
                echo "git@github.com:owner/repo.git"
            "#,
                git_args.display()
            ),
        );
        env_mock::create_mock_bin(
            "gh",
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" >> "{}"
                if [ "$1" = "label" ] && [ "$2" = "list" ]; then
                    echo ""
                else
                    exit 0
                fi
            "#,
                gh_args.display()
            ),
        );

        let _guard = unsafe { env_mock::PathGuard::new(&bin_path) };

        run(LabelsDeployArgs { repo: None })?;

        let gh_cmds = fs::read_to_string(gh_args)?;
        assert!(gh_cmds.contains("label create bugs"));
        assert!(!gh_cmds.contains("label delete"));

        Ok(())
    }

    #[test]
    #[serial]
    #[allow(unused_unsafe)]
    fn deploys_labels_with_replacements() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let git_args = temp_dir.path().join("git_args.txt");
        let gh_args = temp_dir.path().join("gh_args.txt");

        let bin_path = env_mock::create_mock_bin(
            "git",
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" >> "{}"
                echo "git@github.com:owner/repo.git"
            "#,
                git_args.display()
            ),
        );
        env_mock::create_mock_bin(
            "gh",
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" >> "{}"
                if [ "$1" = "label" ] && [ "$2" = "list" ]; then
                    echo "bugs"
                else
                    exit 0
                fi
            "#,
                gh_args.display()
            ),
        );

        let _guard = unsafe { env_mock::PathGuard::new(&bin_path) };

        run(LabelsDeployArgs { repo: Some("owner/repo".to_string()) })?;

        let gh_cmds = fs::read_to_string(gh_args)?;
        assert!(gh_cmds.contains("label delete bugs"));
        assert!(gh_cmds.contains("label create bugs"));

        Ok(())
    }
}
