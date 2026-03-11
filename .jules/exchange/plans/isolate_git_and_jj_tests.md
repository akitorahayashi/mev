---
label: "tests"
---

## Goal

Decouple `git` and `jj` tests from host configurations.

## Problem

Adapter tests rely directly on the local machine state (e.g., `git config user.name`), causing non-determinism and flakes across different test environments.

## Affected Areas

### Adapters

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`

## Constraints

- Filesystem and I/O boundaries in tests must be strictly isolated using temporary directories or dependency injection to prevent uncontrolled side effects on the host system.

## Risks

- None.

## Acceptance Criteria

- `git` and `jj` adapter tests run deterministically in completely isolated setups without referencing the host's actual global configuration.

## Implementation Plan

1. In `tests/adapters/git.rs`, use `tempfile::tempdir()` to create a temporary home directory.
2. Update `git_cli_get_identity_returns_strings` test to set the `HOME` environment variable to the temp directory before invoking the test subject, and unset it or use a scoped environment to avoid side-effects.
3. In `tests/adapters/jj.rs`, do the same for `jj_cli_is_available_returns_bool` if necessary (though it just calls `is_available()`, setting `HOME` ensures complete isolation).
4. Run `cargo test` to verify.
