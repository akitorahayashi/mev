---
label: "tests"
implementation_ready: false
---

## Goal

Ensure tests modifying global state use the `serial_test` crate (`#[serial]`) to avoid test flakiness and race conditions.

## Problem

Some tests implicitly configure global process state, leading to non-determinism, undefined behaviors, or test data races. `crates/mev-internal/src/testing/env_mock.rs` globally calls `env::set_current_dir(target_dir).unwrap();`, which causes failures in parallel `cargo test` execution unless serialized.

## Evidence

- source_event: "global_state_modification_flakiness_qa.md"
  path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "15-16"
  note: "`DirGuard::new` calls `env::set_current_dir(target_dir).unwrap();`."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/tests/gh_contracts.rs`

## Constraints

- Tests modifying global state must use `#[serial]` or be refactored to use DI.

## Acceptance Criteria

- `env_mock.rs` safely isolates its side effects or tests calling it are properly marked with `#[serial]`.