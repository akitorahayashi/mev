---
label: "tests"
---

## Goal
Ensure core domain invariants (`ExecutionPlan` and `RepositoryRef`) have complete isolated unit test coverage.

## Current State
- `src/domain/execution_plan.rs`: This file constructs deterministic ansible execution plans based on provided profiles and tags. It currently has no `#[cfg(test)]` module validating its plan creation logic (`full_setup` and `make`), lacking test coverage.
- `crates/mev-internal/src/domain/repository_ref.rs`: This module parses Git remote URLs and CLI arguments into structural repository references. The code is structurally correct but its line coverage stands at 0/48 lines since parsing logic (e.g. `parse_scp_like_remote` and `parse_ssh_remote`) is untested.

## Plan
1. Add `#[cfg(test)]` module to `src/domain/execution_plan.rs`:
   - Write tests for `ExecutionPlan::full_setup` asserting that the returned plan has the correct profile, `verbose` flag, and contains all tags from `FULL_SETUP_TAGS`.
   - Write tests for `ExecutionPlan::make` asserting that the returned plan matches the provided profile, explicitly supplied tags, and `verbose` flag.
2. Add unit tests to `crates/mev-internal/src/domain/repository_ref.rs`:
   - Add a test for `parse_scp_like_remote` via `RepositoryRef::from_remote_url` for SSH SCP-like format (`git@github.com:owner/repo.git`).
   - Add a test for `parse_ssh_remote` via `RepositoryRef::from_remote_url` for standard SSH format (`ssh://git@github.com/owner/repo.git`).
   - Add a test for `parse_https_remote` via `RepositoryRef::from_remote_url` using the HTTP format (`http://github.com/owner/repo.git`).
   - Add test coverage for invalid remote URL failures.
   - Add test coverage for invalid repo arg references failures.
3. Execute test suite
   - Run `just test` to verify everything is working.
4. Execute pre-commit steps:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria
- `ExecutionPlan` tagging variations are validated by unit tests.
- `RepositoryRef` parsing functions achieve 100% logic path coverage.