//! CLI contract tests for the `switch` command.

use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn switch_help_shows_identity_argument() {
    let ctx = TestContext::new();
    ctx.cli()
        .args(["switch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("IDENTITY"));
}

#[test]
fn switch_alias_sw_is_accepted() {
    let ctx = TestContext::new();
    ctx.cli()
        .args(["sw", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("IDENTITY"));
}

#[test]
fn switch_requires_identity_argument() {
    let ctx = TestContext::new();
    ctx.cli()
        .arg("switch")
        .assert()
        .failure()
        .stderr(predicate::str::contains("IDENTITY").or(predicate::str::contains("required")));
}

#[test]
fn switch_without_config_fails_gracefully() {
    let ctx = TestContext::new();
    ctx.cli().args(["switch", "invalid"]).assert().failure();
}

#[test]
fn switch_help_visible_in_main_help() {
    let ctx = TestContext::new();
    ctx.cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("switch"))
        .stdout(predicate::str::contains("sw"));
}

#[test]
fn switch_success_with_git_and_jj() {
    let ctx = TestContext::new();

    let id_file = ctx.work_dir().join(".config/mev/identity.json");
    std::fs::create_dir_all(id_file.parent().unwrap()).unwrap();
    std::fs::write(
        &id_file,
        r#"{"personal":{"name":"","email":""},"work":{"name":"John Doe","email":"john@example.com"}}"#,
    )
    .unwrap();

    let cmd_log = ctx.work_dir().join("cmd_log.txt");

    let git_bin = ctx.work_dir().join("git");
    std::fs::write(&git_bin, "#!/bin/sh\necho \"git $@\" >> \"$CMD_LOG\"\nexit 0\n").unwrap();
    let mut perms = std::fs::metadata(&git_bin).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&git_bin, perms).unwrap();

    let jj_bin = ctx.work_dir().join("jj");
    std::fs::write(&jj_bin, "#!/bin/sh\necho \"jj $@\" >> \"$CMD_LOG\"\nexit 0\n").unwrap();
    let mut perms = std::fs::metadata(&jj_bin).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&jj_bin, perms).unwrap();

    let path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", ctx.work_dir().display(), path);

    ctx.cli()
        .env("PATH", new_path)
        .env("CMD_LOG", &cmd_log)
        .args(["switch", "work"])
        .assert()
        .success();

    let log_content = std::fs::read_to_string(cmd_log).unwrap();
    assert!(log_content.contains("git config --global user.name John Doe"));
    assert!(log_content.contains("jj config set --user user.name John Doe"));
}

#[test]
fn switch_fails_if_identity_not_configured() {
    let ctx = TestContext::new();

    let id_file = ctx.work_dir().join(".config/mev/identity.json");
    std::fs::create_dir_all(id_file.parent().unwrap()).unwrap();
    std::fs::write(
        &id_file,
        r#"{"personal":{"name":"","email":""},"work":{"name":"","email":""}}"#,
    )
    .unwrap();

    ctx.cli()
        .args(["switch", "work"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("work identity is not configured"));
}
