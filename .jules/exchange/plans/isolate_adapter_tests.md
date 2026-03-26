---
label: "tests"
---

## Goal

Ensure adapter contract tests use mock tools rather than unconstrained host global state. `git` and `jj` CLI tests should use dependency injection to execute fake scripts instead of relying on the host system's binaries.

## Current State

- `src/adapters/git/cli.rs`: Hardcodes the `git` binary name in `Command::new("git")` and its availability check, preventing the injection of fake executables for testing.
- `src/adapters/jj/cli.rs`: Hardcodes the `jj` binary name in `Command::new("jj")` and its availability check, preventing test fakes.
- `tests/adapters/git.rs`: Verifies behavior using the host's `git` installation. Will fail or produce unpredictable results if `git` is absent.
- `tests/adapters/jj.rs`: Verifies behavior using the host's `jj` installation or skips testing entirely if `jj` is absent, masking coverage.

## Plan

1. Modify `src/adapters/git/cli.rs` to support binary path injection.
   - Add `pub bin_path: Option<std::path::PathBuf>` to the `GitCli` struct.
   - Update `GitCli::command` to use `self.bin_path.as_deref().unwrap_or(std::path::Path::new("git"))` instead of `"git"`.
   - Update `GitCli::is_available` to check the configured `bin_path` instead of hardcoding "git". Verify its internal implementation first.

2. Modify `src/adapters/jj/cli.rs` to support binary path injection.
   - Add `pub bin_path: Option<std::path::PathBuf>` to the `JjCli` struct.
   - Update `JjCli::command` to use `self.bin_path.as_deref().unwrap_or(std::path::Path::new("jj"))` instead of `"jj"`.
   - Update `JjCli::is_available` to check the configured `bin_path` instead of hardcoding "jj". Verify its internal implementation first.

3. Update `tests/adapters/git.rs` to use a test fake script.
   - Add a helper function `write_fake_git(dir: &std::path::Path) -> std::path::PathBuf` that writes a mock script acting like the git commands used by `GitCli`.
   - Ensure you verify the exact commands `GitCli` invokes (e.g. `config --global user.name` and `config --global user.email`). The fake script must handle these appropriately. It should write updates to a temporary file (e.g. `$HOME/.fake_git_name`) if it receives arguments. Handle both Unix (`#!/bin/sh`) and Windows (`.bat`).
   - For Unix, ensure the script is executable using `std::os::unix::fs::PermissionsExt`.
   - Refactor `git_cli_reports_available`, `git_cli_get_identity_returns_strings`, and `git_cli_set_identity_updates_config` to inject `bin_path: Some(fake_git_path)` and remove reliance on host `.gitconfig`.
   - Remove any early returns or skips based on host `git` availability.

4. Update `tests/adapters/jj.rs` to use a test fake script.
   - Add a helper function `write_fake_jj(dir: &std::path::Path) -> std::path::PathBuf` that writes a mock script acting like the jj commands used by `JjCli`.
   - Ensure you verify the exact commands `JjCli` invokes (e.g. `config set --user` and the read command). The fake script must handle these appropriately, saving and loading from a temporary file (e.g. `$HOME/.fake_jj_name`). Handle both Unix (`#!/bin/sh`) and Windows (`.bat`).
   - For Unix, ensure the script is executable using `std::os::unix::fs::PermissionsExt`.
   - Refactor `jj_cli_is_available_returns_bool`, `jj_cli_get_identity_returns_configured_values`, and `jj_cli_set_identity_updates_config` to inject `bin_path: Some(fake_jj_path)` and remove reliance on host `.jjconfig.toml`.
   - Remove early returns based on host `jj` availability.

## Acceptance Criteria

- `GitCli` and `JjCli` accept an optional `bin_path`.
- `tests/adapters/git.rs` explicitly injects a fake `git` executable.
- `tests/adapters/jj.rs` explicitly injects a fake `jj` executable.
- Tests no longer require `git` or `jj` to be installed on the host.

## Risks

- Platform-specific issues with the mock scripts (e.g., executing a bash script on Windows). By using `.bat` files on Windows, we mitigate this risk.