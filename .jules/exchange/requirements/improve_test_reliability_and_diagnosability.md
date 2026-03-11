---
label: "tests"
implementation_ready: false
---

## Goal

Decouple adapter test execution from host dependencies to prevent non-determinism, and improve test assertion granularity so input validation failures clearly identify specific violated constraints.

## Problem

Adapter tests for Git and Jujutsu CLIs rely directly on the local machine state (e.g., `git config user.name`), causing non-determinism and flakes across different test environments. In addition, input validation tests use excessively broad assertions (e.g., checking if stderr contains generic terms like "error"), which mask underlying bugs and make failures hard to diagnose.

## Evidence

- source_event: "adapter_tests_host_state_qa.md"
  path: "tests/adapters/git.rs"
  loc: "14"
  note: "`get_identity` calls the real git CLI, dependent on host git config."
- source_event: "adapter_tests_host_state_qa.md"
  path: "tests/adapters/jj.rs"
  loc: "9"
  note: "`is_available` invokes the real jj CLI and silently ignores the outcome, indicating host state pollution."
- source_event: "input_validation_assertion_granularity_qa.md"
  path: "tests/security/input_validation.rs"
  loc: "14"
  note: "`create_rejects_invalid_profile` asserts on any string containing 'error', which masks failures like invalid configuration loading."
- source_event: "input_validation_assertion_granularity_qa.md"
  path: "tests/security/input_validation.rs"
  loc: "25"
  note: "`switch_rejects_invalid_profile` uses the same non-specific assertion for an invalid profile."

## Change Scope

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`
- `tests/security/input_validation.rs`

## Constraints

- Filesystem and I/O boundaries in tests must be strictly isolated using temporary directories or dependency injection to prevent uncontrolled side effects on the host system.
- Tests must assert externally observable behavior at the owning boundary.

## Acceptance Criteria

- `git` and `jj` adapter tests run deterministically in completely isolated, mock, or temporary directory setups without referencing the host's actual global configuration.
- Assertions in `tests/security/input_validation.rs` strictly assert on exact error sub-strings or structured error objects denoting the exact validation failure (e.g., "invalid profile").
