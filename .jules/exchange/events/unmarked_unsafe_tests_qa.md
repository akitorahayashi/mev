---
label: "tests"
created_at: "2024-03-31"
author_role: "qa"
confidence: "high"
---

## Problem

Global state modifications are not explicitly marked as thread-unsafe, and tests using global state aren't always run serially. `crates/mev-internal/src/testing/env_mock.rs` defines `DirGuard` which modifies `std::env::set_current_dir` globally. `tests/adapters/git.rs` modifies `std::env::set_var` / `std::env::set_current_dir` indirectly. `tests/harness/test_context.rs` isolates PATH via `Command`, but internal adapter tests like `crates/mev-internal/src/adapters/git.rs` and `crates/mev-internal/src/adapters/gh.rs` directly modify the mock `PATH` or current directory globally without `#serial` bounds, potentially leading to flakiness.

## Goal

Identify tests using global state overrides (e.g., in `env_mock` and `DirGuard`) and ensure they are explicitly marked as `#[serial]` to prevent flaky test execution, and that any global state operations explicitly use `unsafe` and `#[allow(unused_unsafe)]` to emphasize their non-deterministic nature.

## Context

Running tests concurrently is the default in Rust. Modifying global environment variables like `PATH` or current directory within concurrent execution can interfere, causing non-deterministic test failures (flakes). The architecture strictly requires explicitly isolating global modifications to serial execution, and wrapping modifications in `unsafe` blocks.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 15-23"
  note: "`DirGuard::new()` and `DirGuard::drop()` modify global process state (`env::set_current_dir`) without an `unsafe` block or `#[allow(unused_unsafe)]` annotation."

- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "tests module"
  note: "Tests in `crates/mev-internal/src/adapters/git.rs` use mock binaries and potentially global state (e.g., `git` bin location) but lack `#[serial]` markings."

- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "tests module"
  note: "Tests in `crates/mev-internal/src/adapters/gh.rs` lack `#[serial]` markings while relying on environmental mocking structures."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
