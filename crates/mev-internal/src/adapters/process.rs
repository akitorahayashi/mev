//! Process execution adapter.

use std::process::{Command, Output};

pub fn run_status(
    mut command: Command,
    description: &str,
) -> Result<(), crate::domain::error::InternalError> {
    let status = command.status()?;
    if status.success() {
        return Ok(());
    }

    let message = match status.code() {
        Some(code) => format!("{description} exited with code {code}"),
        None => format!("{description} was terminated by a signal"),
    };

    Err(crate::domain::error::InternalError::Process { message, exit_code: status.code() })
}

pub fn run_output(
    mut command: Command,
    description: &str,
) -> Result<Output, crate::domain::error::InternalError> {
    let output = command.output()?;
    if output.status.success() {
        return Ok(output);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(crate::domain::error::InternalError::Process {
        message: format!("{description} failed: {}", stderr.trim()),
        exit_code: output.status.code(),
    })
}
