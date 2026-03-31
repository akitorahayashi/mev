---
label: "tests"
implementation_ready: false
---

## Goal

Ensure functions modifying global process state in tests are explicitly wrapped in `unsafe { ... }` blocks and annotated with `#[allow(unused_unsafe)]`.

## Problem

Global state modifications are not explicitly marked as thread-unsafe, and tests using global state aren't always run serially.

## Context

Running tests concurrently is the default in Rust. Modifying global environment variables like `PATH` or current directory within concurrent execution can interfere, causing non-deterministic test failures (flakes). The architecture strictly requires explicitly isolating global modifications to serial execution, and wrapping modifications in `unsafe` blocks.
Modifying global state in Rust is intrinsically unsafe in a multi-threaded context like `cargo test`. Memory specifically dictates: "When using functions that modify global process state (e.g., `env::set_current_dir`) in tests, explicitly wrap them in an `unsafe { ... }` block and annotate with `#[allow(unused_unsafe)]` to emphasize their thread-unsafe nature...". `PathGuard` and `DirGuard` are used in parallel test runs unless serialized.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 15-23"
  note: "`DirGuard::new()` and `DirGuard::drop()` modify global process state (`env::set_current_dir`) without an `unsafe` block or `#[allow(unused_unsafe)]` annotation."
- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 47, 56, 61"
  note: "`env::set_var` and `env::remove_var` are used with `unsafe` but missing `#[allow(unused_unsafe)]` annotations"
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

## Constraints

- Follow Testing Rule (Thread Safety Marking).

## Acceptance Criteria

- Functions modifying global state use `unsafe { ... }` blocks and `#[allow(unused_unsafe)]`.
- Tests executing these paths use `#[serial]`.
