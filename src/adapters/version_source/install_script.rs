//! Version source based on the repository install script.

use std::process::Command;

use crate::domain::error::AppError;
use crate::domain::ports::version_source::VersionSource;

const INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/akitorahayashi/mev/main/install.sh";

pub struct InstallScriptVersionSource;

impl VersionSource for InstallScriptVersionSource {
    fn current_version(&self) -> Result<String, AppError> {
        Ok(env!("CARGO_PKG_VERSION").to_string())
    }

    fn run_upgrade(&self) -> Result<(), AppError> {
        println!("Upgrading {} via install script...", env!("CARGO_PKG_NAME"));

        let script = format!("set -euo pipefail; curl -fsSL {INSTALL_SCRIPT_URL} | bash");
        let status = Command::new("bash").args(["-c", &script]).status().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::Update("bash not found. Please ensure bash is installed.".to_string())
            } else {
                AppError::Update(format!("failed to run install script: {e}"))
            }
        })?;

        if !status.success() {
            return Err(AppError::Update(format!(
                "install script failed with exit code {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }
}
