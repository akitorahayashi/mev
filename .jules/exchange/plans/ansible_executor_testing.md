---
label: "tests"
---

## Goal

Improve test coverage for `AnsibleAdapter` to detect regressions in external process orchestration, error handling, and playbook argument formation without running actual long-running playbooks.

## Current State

- `src/adapters/ansible/executor.rs`: The file currently lacks test coverage for `resolve_ansible_playbook_bin` and the command building logic inside `run_playbook`. The execution of subprocess is tightly coupled with command argument building, making it impossible to unit test command formatting without side effects.
- Tests: There are currently no unit tests in `src/adapters/ansible/executor.rs` for `resolve_ansible_playbook_bin` or `run_playbook` argument formatting.
- Documentation: There is no documented strategy for testing `AnsiblePort` implementations or the adapter layer.

## Plan

1. Refactor `src/adapters/ansible/executor.rs`:
   - Extract the command-building logic from `run_playbook` into a new `pub(crate)` method `build_command(&self, profile: &str, tags: &[String], verbose: bool) -> Result<Command, AppError>`.
   - Update `run_playbook` to use this new method and then execute the returned `Command`.
2. Add Unit Tests in `src/adapters/ansible/executor.rs`:
   - Test level and location: Add a `mod tests` block at the bottom of the file containing pure unit tests.
   - Determinism requirements and fixture setup: Use environment manipulation to simulate different scenarios for `resolve_ansible_playbook_bin` (e.g., `ANSIBLE_PLAYBOOK_BIN_ENV` set to a valid path, set to an invalid path, unset). Test state must be isolated.
   - Test `resolve_ansible_playbook_bin`:
     - Pass signal: Returns `Ok(PathBuf)` pointing to the configured path when valid environment variables are set.
     - Fail signal: Returns `Err(AppError::AnsibleExecution)` when paths do not exist.
   - Test `build_command`:
     - Pass signal: The generated `Command` contains the expected arguments (`profile=...`, `--tags`, `-vvv`, `config_dir_abs_path`, etc.). Note: Since `std::process::Command` argument inspection is limited, extracting just the arguments into a `Vec<String>` from a separate function might be more testable, but validating the struct is acceptable if possible.
     - Fail signal: The generated command is missing required arguments or contains incorrect values based on the input parameters.
3. Documentation Updates:
   - Update the inline module documentation in `src/adapters/ansible/executor.rs` to describe the testing strategy for the adapter (e.g. testing command generation separately from execution).

## Acceptance Criteria

- `AnsibleAdapter` argument and command generation logic is thoroughly covered by unit tests.
- `resolve_ansible_playbook_bin` resolution logic is tested against various environment configurations deterministically.
- Tests assert externally observable behavior (the generated command structure) at the owning boundary (`build_command`).
- Tests do not execute actual long-running ansible playbooks.
- Test coverage for `src/adapters/ansible/executor.rs` increases significantly.

## Risks

- Environment variable modification in concurrent tests could lead to flaky behavior. Tests modifying the environment must be run serially (e.g., using `serial_test` crate if available, or carefully managing test thread execution).
