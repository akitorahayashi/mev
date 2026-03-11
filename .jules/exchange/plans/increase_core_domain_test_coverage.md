---
label: "tests"
---

## Goal

Increase test coverage across critical domain and adapter paths to exceed the 40% threshold, ensuring regression risks are mitigated and safety gates pass in CI/CD.

## Problem

Code coverage is around 21%, which is below the 40.00% failure threshold configured in the `just coverage` target. Critical domain transitions and decision paths in `repo_target.rs` and `repository_ref.rs` have very low line coverage, creating high regression risk in core routing and identification logic. Adapter components (`gh`, `git`, `process`) also have missing coverage.

## Affected Areas

### Domain
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/repo_target.rs`

### Adapters
- `crates/mev-internal/src/adapters/git.rs`

## Constraints

- Domain logic tests must reside as self-contained unit tests within their respective `src/domain/` modules inside a `#[cfg(test)]` block.
- Filesystem and I/O boundaries in tests must be strictly isolated using temporary directories or dependency injection.
- Tests must be self-contained with no reliance on external services or mutable shared state.

## Risks

- Testing `adapters/git.rs` commands that modify global environment variables or the global file system could cause race conditions in concurrent tests or side effects in the developer environment. Proper isolation using temporary directories and `Command::current_dir` is required.

## Acceptance Criteria

- `crates/mev-internal/src/domain/repository_ref.rs` contains unit tests for its core logic covering invalid inputs and unsupported remotes.
- `crates/mev-internal/src/domain/repo_target.rs` contains unit tests for its core logic covering the fallback failure state.
- `crates/mev-internal/src/adapters/git.rs` contains isolated tests.
- `just coverage` passes, achieving at least 40.00% coverage overall.

## Implementation Plan

1. Add Tests to `repository_ref.rs`
   - Read `crates/mev-internal/src/domain/repository_ref.rs`.
   - Use `replace_with_git_merge_diff` to add test cases checking invalid formats in `from_repo_arg` (e.g., too many parts) and unsupported/malformed URLs in `from_remote_url` in the existing `#[cfg(test)]` block.
   - Verify the changes using `read_file` or `cat`.
2. Add Tests to `repo_target.rs`
   - Read `crates/mev-internal/src/domain/repo_target.rs`.
   - Use `replace_with_git_merge_diff` to add a test case testing `resolve_repo_ref(None, None)` and verifying the returned error in the existing `#[cfg(test)]` block.
   - Verify the changes using `read_file` or `cat`.
3. Refactor and Add Tests to `adapters/git.rs`
   - Read `crates/mev-internal/src/adapters/git.rs`.
   - Use `replace_with_git_merge_diff` to add a `#[cfg(test)]` mod.
   - Create a helper test function that sets up a temporary git repository using `tempfile` and modifies `git_command` to accept a path to run in the temporary directory. Test methods like `current_origin_url` and `remove_submodule_config_section`.
   - Verify the changes using `read_file` or `cat`.
4. Run Test Suite and Ensure Coverage
   - Run `cargo test` to execute all tests.
   - Run `just coverage` (or equivalent tool) to ensure tests hit the required thresholds.
5. Complete Pre-commit Steps
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
