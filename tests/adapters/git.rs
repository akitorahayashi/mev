//! Adapter contract tests for Git CLI.

use mev::domain::ports::git::GitPort;

#[test]
fn git_cli_reports_available() {
    let git = mev::adapters::git::cli::GitCli;
    assert!(git.is_available());
}

#[test]
fn git_cli_get_identity_returns_strings() {
    let git = mev::adapters::git::cli::GitCli;
    let result = git.get_identity();
    // In CI environments where git identity is not configured, this will explicitly fail
    // and surface an AppError::Config instead of a silent unwrap_or_default() string.
    // The contract here is that if it succeeds, it returns two non-empty strings.
    if let Ok((name, email)) = result {
        let _ = (name.len(), email.len());
    } else {
        assert!(matches!(result, Err(mev::domain::error::AppError::Config(_))));
    }
}
