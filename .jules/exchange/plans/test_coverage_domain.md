---
label: "tests"
---

## Goal

Add comprehensive unit test coverage to the domain module in `crates/mev-internal` to validate externally observable behaviors and constraints of its models.

## Current State

The following structs and modules in `crates/mev-internal/src/domain/` lack sufficient test coverage, leaving critical parsing logic, boundary validations, and reference resolutions unguarded against regressions.
- `crates/mev-internal/src/domain/repository_ref.rs`: Handles parsing `owner/repo` and various git remote URLs into an authoritative github format. Currently reports 0% coverage on critical paths despite having internal unit tests (likely due to module inclusion or target execution issues during coverage runs, but we will augment the tests).
- `crates/mev-internal/src/domain/submodule_path.rs`: Validates submodule paths. Also currently reports 0% test coverage.
- `crates/mev-internal/src/domain/repo_target.rs`: Resolves repository targets preferring explicit values over origin fallbacks. Currently reports 0% test coverage.

## Plan

1. Verify and enhance test coverage for `repository_ref.rs`
   - Ensure `crates/mev-internal/src/domain/repository_ref.rs` tests cover all paths of `from_repo_arg`, `from_remote_url`, `as_gh_repo_arg`, and validation of empty names/owners. Note: The file already contains some unit tests (`#[cfg(test)] mod tests { ... }`). I will review them, add any missing edge cases (like the empty owner/name rejection), and make sure they execute cleanly.
2. Verify and enhance test coverage for `submodule_path.rs`
   - Review the existing unit tests in `crates/mev-internal/src/domain/submodule_path.rs`. Add any missing edge cases for path traversal, absolute paths, and empty paths. Ensure `cargo test` executes them correctly.
3. Verify and enhance test coverage for `repo_target.rs`
   - Review the existing unit tests in `crates/mev-internal/src/domain/repo_target.rs`. Ensure test combinations of both `explicit_repo` and `origin_url` (Some/Some, Some/None, None/Some, None/None) are covered and returning correct `RepositoryRef` instances or explicit errors.
4. Delete requirement document
   - Delete `.jules/exchange/requirements/test_coverage_domain.md`
5. Pre-commit
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
6. Submit
   - Submit the branch via `submit`.

## Acceptance Criteria

- Code coverage metrics for `repository_ref.rs`, `submodule_path.rs`, and `repo_target.rs` are confirmed to be actively collected and passing via `cargo tarpaulin` (i.e., `just coverage` reporting nonzero/satisfactory coverage for these files).
- Missing test scenarios are added for all edge cases mentioned.

## Risks

- Coverage collection via `tarpaulin` may inherently have issues discovering module tests if the modules aren't correctly utilized or exposed. The plan relies on correctly ensuring the test definitions are seen by the coverage tool.
