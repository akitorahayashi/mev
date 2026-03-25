#!/bin/bash
cat << 'INNEREOF' > tests/cli/backup.rs
//! CLI contract tests for the `backup` command.

use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn backup_help_shows_target_argument() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["backup", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("target"));
}

#[test]
fn backup_alias_bk_is_accepted() {
    // The alias should resolve without "unrecognized subcommand" errors.
    // It will fail due to missing ansible assets, but it should not fail
    // on subcommand resolution.
    let ctx = TestContext::new();

    ctx.cli()
        .args(["bk", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available backup targets"));
}

#[test]
fn backup_list_shows_targets() {
    let ctx = TestContext::new();

    ctx.cli().args(["backup", "--list"]).assert().success().stdout(
        predicate::str::contains("system")
            .and(predicate::str::contains("vscode"))
            .and(predicate::str::contains("Available backup targets")),
    );
}

#[test]
fn backup_short_list_flag_shows_targets() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["backup", "-l"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available backup targets"));

    ctx.cli()
        .args(["backup", "--ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available backup targets"));
}

#[test]
fn backup_unknown_target_fails() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["backup", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("is not a valid target"));
}

#[test]
fn backup_help_visible_in_main_help() {
    let ctx = TestContext::new();

    ctx.cli().arg("--help").assert().success().stdout(predicate::str::contains("backup"));
}

#[test]
fn backup_system_success() {
    let ctx = TestContext::new();

    let defs_dir = ctx.work_dir().join(".config/mev/roles/system/common/definitions");
    std::fs::create_dir_all(&defs_dir).unwrap();
    std::fs::write(
        defs_dir.join("test.yml"),
        r#"[{ "key": "AppleShowAllFiles", "type": "bool", "default": false }]"#,
    )
    .unwrap();

    let defaults_bin = ctx.work_dir().join("defaults");
    std::fs::write(&defaults_bin, "#!/bin/sh\nexit 0\n").unwrap();
    let mut perms = std::fs::metadata(&defaults_bin).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&defaults_bin, perms).unwrap();

    let path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", ctx.work_dir().display(), path);

    ctx.cli().env("PATH", new_path).args(["backup", "system"]).assert().success();

    let output_file = ctx.work_dir().join(".config/mev/roles/system/common/system.yml");
    assert!(output_file.exists());
    let content = std::fs::read_to_string(output_file).unwrap();
    assert!(content.contains("AppleShowAllFiles"));
}

#[test]
fn backup_vscode_success() {
    let ctx = TestContext::new();

    let code_bin = ctx.work_dir().join("code");
    std::fs::write(&code_bin, "#!/bin/sh\necho \"ms-python.python\"\nexit 0\n").unwrap();
    let mut perms = std::fs::metadata(&code_bin).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&code_bin, perms).unwrap();

    let path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", ctx.work_dir().display(), path);

    ctx.cli().env("PATH", new_path).args(["backup", "vscode"]).assert().success();

    let output_file = ctx.work_dir().join(".config/mev/roles/editor/common/vscode-extensions.json");
    assert!(output_file.exists());
    let content = std::fs::read_to_string(output_file).unwrap();
    assert!(content.contains("ms-python.python"));
}

#[test]
fn backup_system_failure_no_definitions() {
    let ctx = TestContext::new();

    let defs_dir = ctx.work_dir().join(".config/mev/roles/system/common/definitions");
    std::fs::create_dir_all(&defs_dir).unwrap();
    // Directory exists, but no definitions in it

    ctx.cli()
        .args(["backup", "system"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no setting definitions found"));
}
INNEREOF
