---
label: "tests"
---

## Goal

Provide test coverage for external integrations (GitHub, Git, Process) in the `mev-internal` crate to assure resilient responses, verify correct error catching on external system failures, and ensure proper boundary transformations.

## Current State

- `crates/mev-internal/src/adapters/gh.rs`: GitHub CLI adapter currently has 0% test coverage. It lacks validation for CLI argument construction, output parsing, and error handling.
- `crates/mev-internal/src/adapters/git.rs`: Git CLI adapter currently has 0% test coverage. It lacks validation for sub-process execution arguments, directory/file manipulations, and parsing of git output.
- `crates/mev-internal/src/adapters/process.rs`: Process execution adapter currently has 0% test coverage. It lacks validation for success/failure status code handling and standard error extraction on failure.

## Plan

1. Modify `crates/mev-internal/Cargo.toml` to add necessary `dev-dependencies` (such as `tempfile` and `serial_test`), allowing deterministic testing of global state (like `$PATH`) and file system manipulations.
2. In `crates/mev-internal/src/adapters/process.rs`, add a `#[cfg(test)]` module:
   - Test `run_status` with a successful command (e.g., `Command::new("true")` or `Command::new("echo")`).
   - Test `run_status` with a failing command (e.g., `Command::new("false")`), asserting the returned error contains the expected non-zero exit code.
   - Test `run_output` with a successful command, asserting the standard output is captured correctly.
   - Test `run_output` with a failing command that writes to stderr, asserting the stderr content is captured in the resulting error message.
3. In `crates/mev-internal/src/adapters/gh.rs`, add a `#[cfg(test)]` module:
   - Implement a test helper to generate a mocked `gh` executable script in a temporary directory and prepend its path to the `PATH` environment variable. Ensure tests use `#[serial]` to prevent race conditions during global state (`$PATH`) modification.
   - Test `list_label_names` by having the mock `gh` output predefined stdout text, and verify that the adapter correctly parses the text into a `Vec<String>`.
   - Test `create_label` and `delete_label` by using the mock `gh` to verify that the expected CLI arguments are correctly formed and passed by the adapter.
4. In `crates/mev-internal/src/adapters/git.rs`, add a `#[cfg(test)]` module:
   - Implement a similar test helper for a mocked `git` executable in a temporary directory using `$PATH` modification and `#[serial]`.
   - Test `current_origin_url` using a mock `git` that returns a mocked origin URL string.
   - Test `remove_submodule_config_section` ensuring both success cases and handled error cases (like "No such section") behave properly.
   - Test `remove_submodule_module_dir` by creating a temporary directory structure mimicking `.git/modules/<submodule>` and verifying it is successfully removed by the function.
   - Test `delete_submodule_worktree` to verify the correct `git submodule deinit` and `git rm` commands are executed.

## Acceptance Criteria

- Code coverage for `crates/mev-internal/src/adapters/gh.rs` reaches an acceptable level and external GitHub CLI behavior boundaries are verified.
- Code coverage for `crates/mev-internal/src/adapters/git.rs` reaches an acceptable level and Git CLI behavior boundaries and fs modifications are verified.
- Code coverage for `crates/mev-internal/src/adapters/process.rs` reaches an acceptable level and process execution outcomes are verified.
- All tests modifying environment variables (`$PATH`) use `#[serial]` to guarantee deterministic execution.

## Risks

- Manipulating `$PATH` via `env::set_var` in a multi-threaded test runner can lead to intermittent race conditions if `#[serial]` is missed on any test reading or writing the same variable.
- Tests relying on specific system commands like `true`, `false`, or `echo` might fail on environments where these are not available or behave non-standardly, though they are standard on UNIX-like systems.
