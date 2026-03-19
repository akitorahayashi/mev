//! GitHub CLI adapter.

use std::process::Command;

use crate::adapters::process;
use crate::domain::label_catalog::LabelSpec;
use crate::domain::repository_ref::RepositoryRef;

pub fn list_label_names(repo: &RepositoryRef) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = process::run_output(
        build_gh_command(
            &["label", "list", "--limit", "9999", "--json", "name", "--jq", ".[].name"],
            repo,
        ),
        &format!("gh label list --repo {}", repo.as_gh_repo_arg()),
    )?;

    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

pub fn delete_label(
    repo: &RepositoryRef,
    label_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    process::run_status(
        build_gh_command(&["label", "delete", label_name, "--yes"], repo),
        &format!("gh label delete {label_name} --repo {}", repo.as_gh_repo_arg()),
    )
}

pub fn create_label(
    repo: &RepositoryRef,
    label: &LabelSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    process::run_status(
        build_gh_command(
            &[
                "label",
                "create",
                &label.name,
                "--description",
                &label.description,
                "--color",
                &label.color,
            ],
            repo,
        ),
        &format!("gh label create {} --repo {}", label.name, repo.as_gh_repo_arg()),
    )
}

fn build_gh_command(args: &[&str], repo: &RepositoryRef) -> Command {
    let mut command = Command::new("gh");
    command.args(args);
    command.args(["--repo", &repo.as_gh_repo_arg()]);
    command
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use serial_test::serial;
    use tempfile::TempDir;

    use super::*;

    struct PathGuard {
        original_path: String,
    }

    impl Drop for PathGuard {
        fn drop(&mut self) {
            unsafe {
                env::set_var("PATH", &self.original_path);
            }
        }
    }

    fn create_mock_gh(temp_dir: &TempDir, script_content: &str) -> PathGuard {
        let gh_path = temp_dir.path().join("gh");
        fs::write(&gh_path, script_content).unwrap();
        let mut perms = fs::metadata(&gh_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gh_path, perms).unwrap();

        let original_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", temp_dir.path().display(), original_path);
        unsafe {
            env::set_var("PATH", new_path);
        }
        PathGuard { original_path }
    }

    #[test]
    #[serial]
    fn list_label_names_parses_output() {
        let temp_dir = tempfile::tempdir().unwrap();
        let _path_guard = create_mock_gh(
            &temp_dir,
            r#"#!/bin/sh
            echo "bug\nfeature\nhelp wanted"
            "#,
        );

        let repo = RepositoryRef::from_repo_arg("owner/repo").unwrap();
        let labels = list_label_names(&repo).expect("list_label_names should succeed");
        assert_eq!(labels, vec!["bug", "feature", "help wanted"]);
    }

    #[test]
    #[serial]
    fn create_label_executes_correct_command() {
        let temp_dir = tempfile::tempdir().unwrap();
        let args_file = temp_dir.path().join("args.txt");
        let _path_guard = create_mock_gh(
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" > "{}"
                "#,
                args_file.display()
            ),
        );

        let repo = RepositoryRef::from_repo_arg("owner/repo").unwrap();
        let label = LabelSpec {
            name: "bug".to_string(),
            description: "Something isn't working".to_string(),
            color: "d73a4a".to_string(),
        };

        create_label(&repo, &label).expect("create_label should succeed");

        let executed_args = fs::read_to_string(args_file).unwrap();
        assert_eq!(
            executed_args.trim(),
            "label create bug --description Something isn't working --color d73a4a --repo owner/repo"
        );
    }

    #[test]
    #[serial]
    fn delete_label_executes_correct_command() {
        let temp_dir = tempfile::tempdir().unwrap();
        let args_file = temp_dir.path().join("args.txt");
        let _path_guard = create_mock_gh(
            &temp_dir,
            &format!(
                r#"#!/bin/sh
                echo "$@" > "{}"
                "#,
                args_file.display()
            ),
        );

        let repo = RepositoryRef::from_repo_arg("owner/repo").unwrap();
        delete_label(&repo, "bug").expect("delete_label should succeed");

        let executed_args = fs::read_to_string(args_file).unwrap();
        assert_eq!(executed_args.trim(), "label delete bug --yes --repo owner/repo");
    }
}
