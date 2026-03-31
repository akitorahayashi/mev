---
label: "tests"
---

## Goal

Ensure all test functions propagate errors correctly using `Result` and `?` rather than panicking on `unwrap()`.

## Current State

Tests in several adapter and CLI test modules use `.unwrap()` for potentially failing operations like file system I/O, which masks root causes during failures and violates the testing rule requiring explicit error propagation.

- `tests/adapters/git.rs`: Several tests (`git_cli_reports_available`, `git_cli_get_identity_returns_strings`, `git_cli_set_identity_updates_config`) and helper `write_fake_git` use `.unwrap()` directly for `tempfile::tempdir()`, `fs::write()`, `Result::unwrap()`, `fs::metadata()`, and `fs::set_permissions()`.
- `tests/cli/backup.rs`: Tests (`backup_system_success`, `backup_vscode_success`, `backup_system_failure_no_definitions`) use `.unwrap()` for `std::fs::create_dir_all`, `std::fs::write`, and `std::fs::read_to_string`.
- `tests/cli/switch.rs`: Tests (`switch_success_with_git`, `switch_fails_if_identity_not_configured`) use `.unwrap()` for `std::fs::create_dir_all`, `std::fs::write`, `id_file.parent().unwrap()`, and `std::fs::read_to_string`.

## Plan

1. Update `tests/adapters/git.rs`:
   - Change test signatures for `git_cli_reports_available`, `git_cli_get_identity_returns_strings`, `git_cli_set_identity_updates_config` to return `Result<(), Box<dyn std::error::Error>>`. Add `Ok(())` at the end of these functions.
   - Update `write_fake_git` (both `#[cfg(windows)]` and `#[cfg(unix)]` versions) to return `Result<PathBuf, Box<dyn std::error::Error>>`.
   - In `write_fake_git`, replace `.unwrap()` with `?` for `fs::write`, `fs::metadata`, and `fs::set_permissions`, and add `Ok(path)` at the end.
   - In test functions, replace `write_fake_git(temp_dir.path())` with `write_fake_git(temp_dir.path())?`.
   - Replace `.unwrap()` with `?` for `tempfile::tempdir()`, `fs::write`, and `result.unwrap()` or `get_result.unwrap()` in the test functions.
2. Update `tests/cli/backup.rs`:
   - Change test signatures for `backup_system_success`, `backup_vscode_success`, `backup_system_failure_no_definitions` to return `Result<(), Box<dyn std::error::Error>>`. Add `Ok(())` at the end of these functions.
   - Replace `.unwrap()` with `?` for all instances of `std::fs::create_dir_all`, `std::fs::write`, and `std::fs::read_to_string`.
3. Update `tests/cli/switch.rs`:
   - Change test signatures for `switch_success_with_git`, `switch_fails_if_identity_not_configured` to return `Result<(), Box<dyn std::error::Error>>`. Add `Ok(())` at the end of these functions.
   - Replace `.unwrap()` with `?` for all instances of `std::fs::create_dir_all`, `std::fs::write`, `id_file.parent().unwrap()`, and `std::fs::read_to_string`.
   - Replace `id_file.parent().unwrap()` with `id_file.parent().ok_or("No parent")?`.
4. Run `cargo test` to verify changes.

## Acceptance Criteria

- Setup and IO functions in tests use `?` instead of `.unwrap()`.
- Test function signatures must return `Result<(), Box<dyn std::error::Error>>`.

## Risks

- Adjusting fixture helper functions to return `Result` may require changes to the way they are called, which is accounted for in the plan.
