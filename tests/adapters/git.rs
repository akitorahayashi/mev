//! Adapter contract tests for Git CLI.

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use mev::domain::ports::git::GitPort;

#[cfg(windows)]
fn write_fake_git(dir: &Path) -> PathBuf {
    let script_name = "git.bat";
    let path = dir.join(script_name);

    let fake_config = dir.join(".fake_git_config");

    let bat_content = format!(
        r#"@echo off
set FAKE_CONFIG={config_path}

if "%1"=="config" (
    if "%2"=="--global" (
        if "%3"=="user.name" (
            if "%~4"=="" (
                :: read mode
                findstr /C:"user.name=" "%FAKE_CONFIG%" >nul 2>&1
                if not errorlevel 1 (
                    for /F "tokens=2 delims==" %%A in ('findstr /C:"user.name=" "%FAKE_CONFIG%"') do echo %%A
                )
            ) else (
                :: write mode
                echo user.name=%~4 >> "%FAKE_CONFIG%"
            )
            exit /b 0
        )
        if "%3"=="user.email" (
            if "%~4"=="" (
                :: read mode
                findstr /C:"user.email=" "%FAKE_CONFIG%" >nul 2>&1
                if not errorlevel 1 (
                    for /F "tokens=2 delims==" %%A in ('findstr /C:"user.email=" "%FAKE_CONFIG%"') do echo %%A
                )
            ) else (
                :: write mode
                echo user.email=%~4 >> "%FAKE_CONFIG%"
            )
            exit /b 0
        )
    )
)
exit /b 0
"#,
        config_path = fake_config.display()
    );
    fs::write(&path, bat_content).unwrap();

    path
}

#[cfg(unix)]
fn write_fake_git(dir: &Path) -> PathBuf {
    let script_name = "git";
    let path = dir.join(script_name);

    let fake_config = dir.join(".fake_git_config");

    let sh_content = format!(
        r#"#!/bin/sh
FAKE_CONFIG="{config_path}"
touch "$FAKE_CONFIG"

if [ "$1" = "config" ] && [ "$2" = "--global" ]; then
    if [ "$3" = "user.name" ]; then
        if [ -z "$4" ]; then
            # Read
            grep "^user.name=" "$FAKE_CONFIG" | tail -n 1 | cut -d '=' -f 2-
        else
            # Write
            echo "user.name=$4" >> "$FAKE_CONFIG"
        fi
        exit 0
    elif [ "$3" = "user.email" ]; then
        if [ -z "$4" ]; then
            # Read
            grep "^user.email=" "$FAKE_CONFIG" | tail -n 1 | cut -d '=' -f 2-
        else
            # Write
            echo "user.email=$4" >> "$FAKE_CONFIG"
        fi
        exit 0
    fi
fi
exit 0
"#,
        config_path = fake_config.display()
    );
    fs::write(&path, sh_content).unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();

    path
}

#[test]
fn git_cli_reports_available() {
    let temp_dir = tempfile::tempdir().unwrap();
    let fake_git = write_fake_git(temp_dir.path());
    let git = mev::adapters::git::cli::GitCli {
        home_dir: None,
        bin_path: Some(fake_git),
    };
    assert!(git.is_available());
}

#[test]
fn git_cli_get_identity_returns_strings() {
    let temp_dir = tempfile::tempdir().unwrap();
    let fake_git = write_fake_git(temp_dir.path());

    // Write fake config
    let fake_config = temp_dir.path().join(".fake_git_config");
    fs::write(&fake_config, "user.name=Test User\nuser.email=test@example.com\n").unwrap();

    let git = mev::adapters::git::cli::GitCli {
        home_dir: Some(temp_dir.path().to_path_buf()),
        bin_path: Some(fake_git),
    };

    let result = git.get_identity();
    assert!(result.is_ok());
    let (name, email) = result.unwrap();
    assert_eq!(name, "Test User");
    assert_eq!(email, "test@example.com");
}

#[test]
fn git_cli_set_identity_updates_config() {
    let temp_dir = tempfile::tempdir().unwrap();
    let fake_git = write_fake_git(temp_dir.path());

    let git = mev::adapters::git::cli::GitCli {
        home_dir: Some(temp_dir.path().to_path_buf()),
        bin_path: Some(fake_git),
    };

    let result = git.set_identity("New User", "new@example.com");
    assert!(result.is_ok());

    let get_result = git.get_identity();
    assert!(get_result.is_ok());
    let (name, email) = get_result.unwrap();
    assert_eq!(name, "New User");
    assert_eq!(email, "new@example.com");
}
