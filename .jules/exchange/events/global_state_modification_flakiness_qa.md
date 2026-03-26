---
label: "tests"
created_at: "2024-05-30"
author_role: "qa"
confidence: "high"
---

## Problem

Some tests implicitly configure global process state, leading to non-determinism, undefined behaviors, or test data races. In `tests/harness/test_context.rs`, the code configures paths directly instead of leveraging `serial_test` (`#[serial]`) when it has no option but to modify state, but there isn't actually global state modification using `unsafe { env::set_var }` directly. But if a developer were to introduce one based on `cmd.env()`, it is fine. However, a deeper issue lies within the architecture of `test_tmp_dir` inside `TestContext::new()`. Multiple parallel tests use `tempfile::TempDir::new_in(&test_tmp_dir)` which is relatively safe, but multiple `TestContext::new()` runs concurrently could cause resource exhaustion or interference if not careful. The main problem is `TestContext` doesn't enforce serialization for parallel tests doing heavy operations.
Wait, let's look at `crates/mev-internal/src/testing/env_mock.rs` that explicitly warns:
"Note: Tests using this should be marked with `#[serial]` to avoid environment variable races."
And `env::set_current_dir(target_dir).unwrap();` is used globally by `DirGuard`. This modifies global process state, affecting all tests running concurrently.

## Goal

Ensure tests using `DirGuard` or `PathGuard` in `crates/mev-internal/src/testing/env_mock.rs` or any logic modifying global state (`env::set_current_dir`, `env::set_var`) use the `serial_test` crate (`#[serial]`) to avoid test flakiness, or refactor the code to avoid modifying global state entirely.

## Context

Modifying global state in Rust tests is inherently unsafe and leads to race conditions since `cargo test` runs tests in parallel by default on multiple threads. If one test changes the current working directory while another test executes, the second test may fail unpredictably (flakiness). This breaks the core principle of isolation and determinism over retries.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "15-16"
  note: "`DirGuard::new` calls `env::set_current_dir(target_dir).unwrap();`. If any test uses `DirGuard` without `#[serial]`, it causes global state data races."
- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "44-46"
  note: "`PathGuard::new` calls `unsafe { env::set_var(\"PATH\", new_path); }`. It expects tests to use `#[serial]`, but if they forget, undefined behavior occurs."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/tests/gh_contracts.rs` (needs to verify all tests use #[serial] properly or refactor them to use DI)
