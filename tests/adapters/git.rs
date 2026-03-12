//! Adapter contract tests for Git CLI.

use mev::domain::ports::git::GitPort;
use serial_test::serial;
use std::fs;

struct EnvGuard {
    key: String,
    original_value: Option<String>,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        let original_value = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value);
        }
        Self {
            key: key.to_string(),
            original_value,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(ref val) = self.original_value {
                std::env::set_var(&self.key, val);
            } else {
                std::env::remove_var(&self.key);
            }
        }
    }
}

#[test]
#[serial]
fn git_cli_reports_available() {
    let git = mev::adapters::git::cli::GitCli;
    assert!(git.is_available());
}

#[test]
#[serial]
fn git_cli_get_identity_returns_strings() {
    let temp_dir = tempfile::tempdir().unwrap();

    // Create a dummy .gitconfig in the temporary directory
    let gitconfig_content = "[user]\n\tname = Test User\n\temail = test@example.com\n";
    fs::write(temp_dir.path().join(".gitconfig"), gitconfig_content).unwrap();

    let _env_guard = EnvGuard::set("HOME", temp_dir.path().to_str().unwrap());

    let git = mev::adapters::git::cli::GitCli;
    let result = git.get_identity();

    assert!(result.is_ok());
    let (name, email) = result.unwrap();
    assert_eq!(name, "Test User");
    assert_eq!(email, "test@example.com");
}
