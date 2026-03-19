use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn deploy_help_shows_description() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["gh", "labels", "deploy", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploy the bundled label catalog"));
}

#[test]
fn deploy_calls_gh_and_handles_labels_properly() {
    let ctx = TestContext::new();

    let gh_mock = r#"#!/bin/bash
if [[ "$*" == *"label list"* ]]; then
    echo '[{"name": "bug"}, {"name": "enhancement"}]'
elif [[ "$*" == *"label delete"* ]]; then
    echo "deleted"
elif [[ "$*" == *"label create"* ]]; then
    echo "created"
else
    echo "Unexpected args: $*" >&2
    exit 1
fi
"#;

    let git_mock = r#"#!/bin/bash
if [[ "$1" == "remote" ]]; then
    echo "git@github.com:foo/bar.git"
fi
"#;

    let mocks_dir = ctx.work_dir().join("mocks");
    std::fs::create_dir_all(&mocks_dir).unwrap();
    let gh_path = mocks_dir.join("gh");
    let git_path = mocks_dir.join("git");

    std::fs::write(&gh_path, gh_mock).unwrap();
    std::fs::write(&git_path, git_mock).unwrap();

    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&gh_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    std::fs::set_permissions(&git_path, std::fs::Permissions::from_mode(0o755)).unwrap();

    let path_env = std::env::var("PATH").unwrap_or_default();
    let path_env = format!("{}:{}", mocks_dir.display(), path_env);

    ctx.cli()
        .env("PATH", path_env)
        .args(["gh", "labels", "deploy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deployed bundled labels to github.com/foo/bar."));
}
