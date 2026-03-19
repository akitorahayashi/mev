---
label: "refacts"
---

## Goal

Remove `unsafe` environment variable mutations from the test suite to prevent undefined behavior and data races. Ensure `resolve_ansible_playbook_bin` avoids depending solely on process global state or redesign the test so that it avoids global state changes. The preferred approach is to pass environment variables explicitly.

## Current State

- `src/adapters/ansible/executor.rs`: The tests use a custom `EnvGuard` struct that leverages `unsafe { env::set_var(...) }` and `unsafe { env::remove_var(...) }` to inject environment variables (`ANSIBLE_PLAYBOOK_BIN`, `PIPX_HOME`, `HOME`) for testing `resolve_ansible_playbook_bin`. This multi-threading-unsafe mutation of global process state must be removed. `resolve_ansible_playbook_bin` directly reads the process environment using `env::var_os`.

## Plan

1. Modify `resolve_ansible_playbook_bin` in `src/adapters/ansible/executor.rs` to accept environment variables.
   - Change the signature to: `fn resolve_ansible_playbook_bin<'a>(env_vars: impl Iterator<Item = (&'a str, Option<&'a std::ffi::OsStr>)>) -> Result<PathBuf, AppError>` or similar, or better, pass a struct/closure that abstracts environment reads.
   - A simpler approach: Change the function signature to `fn resolve_ansible_playbook_bin_with_env<F>(get_env: F) -> Result<PathBuf, AppError> where F: Fn(&str) -> Option<std::ffi::OsString>`.
   - Implement `resolve_ansible_playbook_bin()` to call `resolve_ansible_playbook_bin_with_env(std::env::var_os)`.
2. Update the `build_command` method in `src/adapters/ansible/executor.rs` to call the updated `resolve_ansible_playbook_bin`. No changes to `build_command` signature.
3. Remove the `EnvGuard` struct and its implementation from `src/adapters/ansible/executor.rs` (in `mod tests`).
4. Update the tests (`test_resolve_ansible_playbook_bin_custom_valid`, `test_resolve_ansible_playbook_bin_custom_invalid`, `test_resolve_ansible_playbook_bin_pipx_home_valid`, `test_resolve_ansible_playbook_bin_home_valid`, `test_resolve_ansible_playbook_bin_not_found`, `test_build_command_success`) to use `resolve_ansible_playbook_bin_with_env` with a closure or mock environment map instead of using `EnvGuard`.
   - E.g., for `test_resolve_ansible_playbook_bin_custom_valid`, create a `HashMap` of mocked environment variables and pass `|k| mock_env.get(k).cloned()` to the function.
   - For `test_build_command_success`, since it calls `build_command` which calls `resolve_ansible_playbook_bin`, we need to find a way to test it without modifying process-wide environment variables, or update `build_command` to take a custom environment resolver as well.
   - Given `build_command` is public(crate) and used in `run_playbook`, it might be better to create an `EnvResolver` trait or struct and pass it down, or update `AnsibleAdapter` to hold an optional mocked environment for testing. However, keeping it simple: modify `build_command` and `resolve_ansible_playbook_bin` to accept an optional mocked environment map, or inject a `HashMap<String, OsString>` into `AnsibleAdapter`.
   - Wait, `resolve_ansible_playbook_bin` does not use `self`. It's a free function.
   - Let's update `AnsibleAdapter` to optionally override environment variables for testing, or update `build_command` to take an environment closure/map, but `run_playbook` calls `build_command`.
   - Actually, a better approach for `AnsibleAdapter`: instead of modifying the adapter structure, just update `resolve_ansible_playbook_bin` to `resolve_ansible_playbook_bin(env: &impl EnvProvider)` and implement it for `std::env` and a mock map.
   - Or, easier: extract environment resolution in `resolve_ansible_playbook_bin` to a generic function `fn resolve_ansible_playbook_bin_with_env(env: impl Fn(&str) -> Option<std::ffi::OsString>) -> Result<PathBuf, AppError>`. `resolve_ansible_playbook_bin()` becomes a wrapper using `std::env::var_os`.
   - Then update `test_resolve_ansible_playbook_bin_*` tests to use `resolve_ansible_playbook_bin_with_env`.
   - For `test_build_command_success`: we still need `AnsibleAdapter::build_command` to use the mocked path for `ansible-playbook`. It currently relies on `resolve_ansible_playbook_bin()`. Since `build_command` calls `resolve_ansible_playbook_bin()`, which uses `std::env::var_os`, we can either add `#[cfg(test)]` block to override `resolve_ansible_playbook_bin` or update `build_command` to take an env closure.
   - Updating `build_command` to `pub(crate) fn build_command_with_env(&self, profile: &str, tags: &[String], verbose: bool, env: impl Fn(&str) -> Option<std::ffi::OsString>)` and making `build_command` call it with `std::env::var_os`.
   - Update `test_build_command_success` to call `build_command_with_env` passing the mocked environment.
5. Verify changes by running `cargo check` and `cargo test`.
6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. Submit the generated implementation plan.

## Acceptance Criteria

- Uses of `unsafe { env::set_var(...) }` and `unsafe { env::remove_var(...) }` are removed from `src/adapters/ansible/executor.rs`.
- `EnvGuard` struct is removed.
- Tests that rely on `EnvGuard` are redesigned to explicitly pass environment configurations.
- `cargo test` passes cleanly without modifying process-wide environment variables.

## Risks

- `test_build_command_success` might break if the mocked environment is not passed correctly through `build_command`.
- Changes to the function signatures might propagate further than expected if not handled via a wrapper for production use (e.g., `build_command` wrapper around `build_command_with_env`).