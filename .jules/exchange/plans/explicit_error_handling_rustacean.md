---
label: "refacts"
---

## Goal
Eliminate silent fallbacks (`unwrap_or_default()`, `.unwrap_or()`) and type-erased errors (`Box<dyn Error>`) across internal crates and adapters, ensuring explicit error classification and propagation.

## Current State
The codebase currently swallows failures or collapses them into generic representations, masking real issues.
- `crates/mev-internal/src/adapters/process.rs`: Uses `Box<dyn std::error::Error>` and `status.code().unwrap_or(1)`.
- `crates/mev-internal/src/adapters/git.rs`: Uses `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/adapters/gh.rs`: Uses `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/domain/repository_ref.rs`: Returns `Box<dyn std::error::Error>` on parse errors.
- `crates/mev-internal/src/domain/repo_target.rs`: Returns `Box<dyn std::error::Error>`
- `crates/mev-internal/src/domain/submodule_path.rs`: Returns `Box<dyn std::error::Error>`
- `crates/mev-internal/src/domain/label_catalog.rs`: Returns `Box<dyn std::error::Error>`
- `src/app/commands/backup/mod.rs`: Handles fallback in formatting logic `serde_json::to_string(&value).unwrap_or(value)` and parsing numbers.
- `src/adapters/ansible/executor.rs`: Swallows signals by using `unwrap_or(-1)` for exit codes and uses `Box<dyn std::error::Error>` in loading roles/catalog.
- `src/adapters/git/cli.rs`: Uses `unwrap_or_default()` when git command fails on reading config.
- `src/adapters/jj/cli.rs`: Uses `unwrap_or_default()` when jj command fails on reading config.
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`: Returns `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`: Returns `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`: Returns `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/cli/gh.rs`: Returns `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/cli/git.rs`: Returns `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/domain/mod.rs`: Needs to export a new `error` module.

## Plan
1. Run `cargo add thiserror` in `crates/mev-internal` directory to add the `thiserror` crate if it does not already exist.
2. Create `crates/mev-internal/src/domain/error.rs` using `write_file` to define an `InternalError` enum deriving `thiserror::Error` for the crate's internal domain errors. Add variants for parsing, process execution, configuration, and JSON serialization. Add the `error` module to `crates/mev-internal/src/domain/mod.rs` using `replace_with_git_merge_diff`.
3. Update `crates/mev-internal/src/domain/repository_ref.rs`, `crates/mev-internal/src/domain/repo_target.rs`, `crates/mev-internal/src/domain/submodule_path.rs`, and `crates/mev-internal/src/domain/label_catalog.rs` using `replace_with_git_merge_diff` to return `Result<..., InternalError>` instead of `Box<dyn Error>`.
4. Update `crates/mev-internal/src/adapters/process.rs` using `replace_with_git_merge_diff` to return `Result<..., InternalError>` and avoid `unwrap_or(1)` for the exit status code by exposing an explicit option, using a custom error variant if the process was terminated by a signal.
5. Update `crates/mev-internal/src/adapters/git.rs` and `crates/mev-internal/src/adapters/gh.rs` using `replace_with_git_merge_diff` to propagate `InternalError` from `process.rs`.
6. Update `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`, `crates/mev-internal/src/app/commands/gh/labels_reset.rs`, and `crates/mev-internal/src/app/commands/git/delete_submodule.rs` using `replace_with_git_merge_diff` to return `Result<..., InternalError>` instead of `Box<dyn std::error::Error>`. Update `crates/mev-internal/src/app/cli/gh.rs` and `crates/mev-internal/src/app/cli/git.rs` and any other command callers to expect the new errors.
7. Update `src/adapters/ansible/executor.rs` using `replace_with_git_merge_diff` to bubble up signal deaths rather than defaulting to `-1`. Ensure `load_catalog` returns `AppError::AnsibleExecution` or `AppError::Config` instead of `Box<dyn Error>`.
8. Update `src/adapters/git/cli.rs` and `src/adapters/jj/cli.rs` using `replace_with_git_merge_diff`: propagate errors in `read_config` using `Result<String, AppError>` instead of `.unwrap_or_default()`. Make `get_identity` fail explicitly when the command fails rather than succeeding with an empty string. Update any callers to handle `AppError`.
9. Update `src/app/commands/backup/mod.rs` using `replace_with_git_merge_diff` to avoid `.unwrap_or` calls when parsing numbers and serializing JSON. Instead, explicitly match successes and handle failures appropriately instead of falling back to default/raw strings silently.
10. Run `just test` to verify no regressions exist.
11. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria
- Type-erased `Box<dyn Error>` usages are replaced with strongly-typed enums using `thiserror`.
- Git, JJ, Ansible, and process executors properly bubble up errors without defaulting (`unwrap_or_default` and `unwrap_or`).
- No silent fallbacks remain in adapters and execution layers.
- All tests pass, preserving behavior with explicit boundary error reporting.

## Risks
- Incorrect `Result` propagation breaking command chaining in `.unwrap_or` paths.
- Changing `mev-internal` error types causing compilation errors in tests or CLI invocations expecting `Box<dyn std::error::Error>`.
