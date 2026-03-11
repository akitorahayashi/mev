---
label: "refacts"
---

## Goal

Replace type-erased errors (`Box<dyn std::error::Error>`) with strongly-typed domain errors (like `AppError`) to preserve error semantics, classifications, and enable proper error handling. Ensure all `unwrap()`/`expect()` usage and silent fallback behaviors (like `.unwrap_or(-1)`) outside test modules are replaced with explicit error handling and propagation using robust domain error types.

## Problem

Widespread use of type-erased `Box<dyn std::error::Error>` across the codebase (specifically in adapters) loses semantic error classification. Furthermore, scattered use of `unwrap()`, `expect()`, and silent fallback patterns (like `.unwrap_or(-1)`) outside of tests bypass error boundaries and mask failures, undermining the principle that errors are part of the contract and must retain their semantic meaning.

## Affected Areas

### Codebase

- `src/adapters/ansible/executor.rs`
- `src/app/container.rs`
- `crates/mev-internal/src/domain/error.rs`
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/app/cli/git.rs`
- `crates/mev-internal/src/app/cli/gh.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `src/adapters/version_source/install_script.rs`

## Constraints

- Errors must preserve semantic meaning and attach context where the system meets the world.
- Silent fallbacks are prohibited. Any fallback must be explicit, opt-in, and surfaced as a failure or a clearly logged, reviewed decision.
- Production paths must safely propagate errors rather than panicking with expect or unwrap.

## Risks

- Changing the error types may cause compilation errors that require updating numerous function signatures and mapping logic across callers.
- Changing error behavior could alter CLI exit code behaviors, affecting any downstream consumers relying on specific error formatting.

## Acceptance Criteria

- Box<dyn std::error::Error> is completely removed from adapter return types and replaced with specific, domain-aligned error types (e.g., AppError or sub-variants).
- unwrap_or(-1) usages for process exit codes are removed and replaced with explicit checking and error forwarding for process execution failures.
- Production domain logic no longer invokes unwrap() or expect().

## Implementation Plan

1. Run `rg "Box<dyn std::error::Error>"` to identify all usages of type-erased errors and ensure we capture all of them before refactoring.
2. Introduce or expand domain-level error types (like `AppError`) in `src/domain/error.rs` or `crates/mev-internal/src/domain/error.rs` to cover new errors like `ProcessExecutionFailed`, `GitOperationFailed`, `GithubOperationFailed`.
3. Update adapter modules (`src/adapters/ansible/executor.rs`, `crates/mev-internal/src/adapters/process.rs`, `crates/mev-internal/src/adapters/git.rs`, `crates/mev-internal/src/adapters/gh.rs`) to return the newly created typed domain errors, mapping raw io/process errors to these domain errors explicitly.
4. Update domain models (`crates/mev-internal/src/domain/repo_target.rs`, `crates/mev-internal/src/domain/repository_ref.rs`, `crates/mev-internal/src/domain/submodule_path.rs`, `crates/mev-internal/src/domain/label_catalog.rs`) to use strongly typed errors and propagate them.
5. Update command entry points (`crates/mev-internal/src/app/cli/git.rs`, `crates/mev-internal/src/app/cli/gh.rs`, `crates/mev-internal/src/app/commands/gh/labels_reset.rs`, `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`, `crates/mev-internal/src/app/commands/git/delete_submodule.rs`, `src/app/container.rs`) to accept the strongly typed errors and bubble them up for proper presentation.
6. Audit production logic for `unwrap()` and `expect()` usage, specifically looking at `crates/mev-internal/src/domain/repository_ref.rs` and other areas mentioned. Refactor these to propagate errors using `?`.
7. Locate and replace all `unwrap_or(-1)` silent fallbacks, specifically in `src/adapters/version_source/install_script.rs` and `src/adapters/ansible/executor.rs`, converting them to properly construct domain error types with explicit failed exit codes.
8. Run `cargo check` and `cargo test` to ensure all type-checks pass, errors are correctly aligned, and test suites complete successfully without logic regressions.
