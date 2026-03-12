//! Adapter contract tests for Jujutsu CLI.

use mev::domain::ports::jj::JjPort;
use serial_test::serial;

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
fn jj_cli_is_available_returns_bool() {
    let temp_dir = tempfile::tempdir().unwrap();
    let _env_guard = EnvGuard::set("HOME", temp_dir.path().to_str().unwrap());

    let jj = mev::adapters::jj::cli::JjCli;
    // May be false in CI; just verify no panic.
    let _ = jj.is_available();
}
