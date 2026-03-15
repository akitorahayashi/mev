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
        .args(["bk", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available backup targets"));
}

#[test]
fn backup_positional_list_shows_targets() {
    let ctx = TestContext::new();

    ctx.cli().args(["backup", "list"]).assert().success().stdout(
        predicate::str::contains("system")
            .and(predicate::str::contains("vscode"))
            .and(predicate::str::contains("Available backup targets")),
    );
}

#[test]
fn backup_unknown_target_fails() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["backup", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown backup target"));
}

#[test]
fn backup_help_visible_in_main_help() {
    let ctx = TestContext::new();

    ctx.cli().arg("--help").assert().success().stdout(predicate::str::contains("backup"));
}

#[test]
fn backup_system_generates_yaml() {
    let ctx = TestContext::new();

    let def_dir = ctx.work_dir().join(".config/mev/roles/system/common/definitions");
    std::fs::create_dir_all(&def_dir).unwrap();

    let def_file = def_dir.join("test.yml");
    std::fs::write(
        &def_file,
        r#"
- key: "test_key"
  type: "string"
  default: "test_value"
"#,
    )
    .unwrap();

    // Create a fake defaults executable that just prints a known value so we don't rely on the real defaults tool
    let bin_dir = ctx.work_dir().join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    let fake_defaults = bin_dir.join("defaults");
    std::fs::write(&fake_defaults, "#!/bin/sh\necho 'fake_value'\n").unwrap();

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&fake_defaults).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_defaults, perms).unwrap();
    }

    let original_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), original_path);

    ctx.cli()
        .args(["backup", "system"])
        .env("PATH", new_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Backup completed successfully"));

    let out_file = ctx.work_dir().join(".config/mev/roles/system/common/system.yml");
    assert!(out_file.exists());
    let content = std::fs::read_to_string(&out_file).unwrap();
    assert!(content.contains("test_key"));
    assert!(content.contains("fake_value"));
}

#[test]
fn backup_system_emits_fallback_warning_to_stderr() {
    let ctx = TestContext::new();

    // Do not create local definitions to trigger the fallback logic

    // We still need `defaults` to pass if it checks real defaults, but without definitions it will just use package defaults,
    // which in TestContext will not exist either, so it will fail with "definitions directory not found".
    // Wait, the fallback resolves to `ansible_dir/roles/system/config/common/definitions`.
    // Let's see what happens without injecting definitions. It will use package defaults.
    // If the test context doesn't map to real ansible_dir, it will fail, BUT it will have emitted the stderr first.
    let result = ctx.cli().args(["backup", "system"]).assert();

    result.stderr(predicate::str::contains("Local definitions not found at"));
}

#[test]
fn backup_vscode_generates_json() {
    let ctx = TestContext::new();

    let bin_dir = ctx.work_dir().join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    let fake_code = bin_dir.join("code");
    std::fs::write(
        &fake_code,
        r#"#!/bin/sh
if [ "$1" = "--list-extensions" ]; then
    echo 'ms-python.python'
    echo 'rust-lang.rust-analyzer'
fi
"#,
    )
    .unwrap();

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&fake_code).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_code, perms).unwrap();
    }

    let original_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), original_path);

    ctx.cli()
        .args(["backup", "vscode"])
        .env("PATH", new_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Backup completed successfully"));

    let out_file = ctx.work_dir().join(".config/mev/roles/editor/common/vscode-extensions.json");
    assert!(out_file.exists());
    let content = std::fs::read_to_string(&out_file).unwrap();
    assert!(content.contains("ms-python.python"));
    assert!(content.contains("rust-lang.rust-analyzer"));
}
