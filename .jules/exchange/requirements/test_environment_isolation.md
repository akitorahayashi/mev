---
label: "tests"
implementation_ready: false
---

## Goal

Ensure tests are isolated, deterministic, and do not manipulate global process state (like `PATH`), instead relying on dependency injection or test fakes.

## Problem

Tests in `crates/mev-internal/src/adapters/git.rs` and `gh.rs` modify global process state (`PATH`), leading to flakiness. Additionally, adapter contract tests (`tests/adapters/git.rs` and `tests/adapters/jj.rs`) blindly depend on host environment setups, making them unconstrained and non-deterministic across different CI setups.

## Evidence

- source_event: "flaky_git_env_tests_qa.md"
  path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "55"
  note: "Modifies global process PATH using unsafe `env::set_var`."
- source_event: "flaky_git_env_tests_qa.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "73"
  note: "Requires `#[serial(env_path)]` macro due to global state modification in tests."
- source_event: "unreliable_adapter_tests_qa.md"
  path: "tests/adapters/git.rs"
  loc: "12"
  note: "Asserts `git.get_identity()` is always OK, which fails if the host has no git user configured."
- source_event: "unreliable_adapter_tests_qa.md"
  path: "tests/adapters/jj.rs"
  loc: "9"
  note: "Just verifies no panic, failing to assert any meaningful behavioral property."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`

## Constraints

- Tests must not use `unsafe { env::set_var(...) }` to modify global process state.
- Adapter contract tests must not depend on unconstrained host global state. Use mock directories with known Git/Jujutsu variables.
- Prefer explicit dependency injection for test environments over global state overrides.

## Acceptance Criteria

- Adapter tests run deterministically in any sandbox environment regardless of host Git/JJ config.
- No tests rely on `unsafe` global `PATH` manipulation.
- `cargo test` runs without intermittent flakiness or mandatory serialization over `env_path`.