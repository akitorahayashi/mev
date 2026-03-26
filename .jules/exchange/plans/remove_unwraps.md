---
label: "refacts"
---

## Current State

The codebase heavily uses `.unwrap()` or `.expect()` in non-test paths (e.g., parsing env variables, locator/executor directories), presenting a risk of unhandled panics and silent failures in critical execution paths.
- `src/adapters/ansible/locator.rs`: `unwrap()` is used in `locate_ansible_dir_with` or related functions to process directories/files, or test setups directly. While evidence points to `98, 99`, the goal is to remove unwraps from all non-test execution paths, as well as fixing the test configurations. Wait, evidence mentions: "Locator functions use unwrap() to create directories or write files directly, bypassing error handling." Although this is inside `#[cfg(test)]`, we must update the test configurations to correctly handle errors, or explicitly acknowledge them if they represent test setups, but the requirement specifies removing them from the specified files. We will update `locator.rs` test functions to return `Result<(), Box<dyn std::error::Error>>` and propagate errors explicitly with `?`.
- `src/adapters/ansible/executor.rs`: Uses `.unwrap_or(-1)` or `.unwrap()` internally. The plan will instruct to locate all instances of `.unwrap()` and `.expect()` in `src/adapters/ansible/executor.rs` and update them to use safe fallbacks or typed errors. Update all test setups to propagate `Result` and use `?`.
- `crates/mev-internal/src/adapters/gh.rs`: Test methods extensively use `.expect()` or `.unwrap()`. We will convert the test functions to return `Result<(), Box<dyn std::error::Error>>` and propagate with `?`.
- `crates/mev-internal/src/adapters/git.rs`: Contains `std::env::current_dir().unwrap()` in `remove_submodule_module_dir()`, which is a production path causing potential panic. Tests also use `unwrap()` and `expect()`.

### Implementation Targets
- `crates/mev-internal/src/adapters/git.rs`: Update `remove_submodule_module_dir` to return `Result` by replacing `unwrap()` with `?`. Update tests to remove `unwrap()`/`expect()`.
- `crates/mev-internal/src/adapters/gh.rs`: Update tests to remove `unwrap()`/`expect()`.
- `src/adapters/ansible/locator.rs`: Update `create_ansible_dir` and test functions to return `Result` and propagate errors explicitly.
- `src/adapters/ansible/executor.rs`: Replace `unwrap()`/`expect()` in test functions with explicit `?` propagation. Ensure no `unwrap()` exists in the production execution path.

### Documentation Targets
- No documentation updates are strictly required for removing `unwrap()` because it does not change the core domain terminology or outward-facing CLI APIs. However, internal rustdoc for `git.rs` or `locator.rs` test helpers should remain accurate regarding the new `Result` return types.

### Test Targets
- The tests for `git`, `gh`, `locator`, and `executor` will themselves be updated to return `Result<(), Box<dyn std::error::Error>>` to verify the execution paths safely handle errors and do not panic on valid inputs. Behavior invariants (e.g. tests validating failure scenarios) must remain unchanged.

## Plan

1. Update `crates/mev-internal/src/adapters/git.rs`
   - In `remove_submodule_module_dir`, change `std::env::current_dir().unwrap()` to `std::env::current_dir()?`.
   - Update all test functions to return `Result<(), Box<dyn std::error::Error>>`. Replace `.unwrap()` and `.expect()` calls with explicit error propagation (`?`).

2. Update `crates/mev-internal/src/adapters/gh.rs`
   - Update all test functions to return `Result<(), Box<dyn std::error::Error>>`. Replace all `.unwrap()` and `.expect()` calls in tests with explicit error propagation (`?`).

3. Update `src/adapters/ansible/locator.rs`
   - Modify the `create_ansible_dir` helper to return `Result<PathBuf, std::io::Error>` and use `?` instead of `.unwrap()`. Update callers to propagate the error.
   - Update all test functions to return `Result<(), Box<dyn std::error::Error>>` and propagate errors explicitly.

4. Update `src/adapters/ansible/executor.rs`
   - Replace any `.unwrap()` and `.expect()` calls in tests with explicit error propagation (`?`). Update test signatures to return `Result<(), Box<dyn std::error::Error>>`.
   - Search for any `.unwrap()`/`.expect()` in the non-test production path of `executor.rs` (e.g. `cmd_result.unwrap()` if it wasn't caught by the regex) and map to explicit errors if found.

5. Run tests
   - Run `cargo test` and `cd crates/mev-internal && cargo test` to verify all updated tests compile and pass successfully, confirming that behavior invariants are preserved.

6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
