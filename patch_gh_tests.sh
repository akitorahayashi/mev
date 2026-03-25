#!/bin/bash
cat << 'INNEREOF' > crates/mev-internal/tests/gh_contracts.rs
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::env;
use std::fs;

#[test]
#[serial(env_path)]
fn test_gh_labels_deploy() {
    let temp_dir = tempfile::tempdir().unwrap();
    let gh_log = temp_dir.path().join("gh_log.txt");

    let mock_script = format!(
        "#!/bin/sh\necho \"\$@\" >> \"{}\"\nif [ \"\$1\" = \"label\" ] && [ \"\$2\" = \"list\" ]; then\n    echo \"bug\"\nelse\n    exit 0\nfi",
        gh_log.display()
    );

    let bin_path = temp_dir.path().join("gh");
    fs::write(&bin_path, mock_script).unwrap();
    let mut perms = fs::metadata(&bin_path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&bin_path, perms).unwrap();

    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", temp_dir.path().display(), original_path);

    let bin = assert_cmd::cargo::cargo_bin("mev-internal");
    let mut cmd = Command::new(bin);
    cmd.env("PATH", new_path);
    cmd.args(["gh", "labels", "deploy", "--repo", "owner/repo"]);

    cmd.assert().success();

    let log_content = fs::read_to_string(gh_log).unwrap();
    assert!(log_content.contains("label list --limit 9999 --json name --jq .[].name --repo owner/repo"));
    assert!(log_content.contains("label delete bug --yes --repo owner/repo"));
    assert!(log_content.contains("label create"));
}

#[test]
#[serial(env_path)]
fn test_gh_labels_reset() {
    let temp_dir = tempfile::tempdir().unwrap();
    let gh_log = temp_dir.path().join("gh_log.txt");

    let mock_script = format!(
        "#!/bin/sh\necho \"\$@\" >> \"{}\"\nif [ \"\$1\" = \"label\" ] && [ \"\$2\" = \"list\" ]; then\n    echo \"bug\"\nelse\n    exit 0\nfi",
        gh_log.display()
    );

    let bin_path = temp_dir.path().join("gh");
    fs::write(&bin_path, mock_script).unwrap();
    let mut perms = fs::metadata(&bin_path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&bin_path, perms).unwrap();

    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", temp_dir.path().display(), original_path);

    let bin = assert_cmd::cargo::cargo_bin("mev-internal");
    let mut cmd = Command::new(bin);
    cmd.env("PATH", new_path);
    cmd.args(["gh", "labels", "reset", "--repo", "owner/repo"]);

    cmd.assert().success();

    let log_content = fs::read_to_string(gh_log).unwrap();
    assert!(log_content.contains("label list --limit 9999 --json name --jq .[].name --repo owner/repo"));
    assert!(log_content.contains("label delete bug --yes --repo owner/repo"));
}
INNEREOF
