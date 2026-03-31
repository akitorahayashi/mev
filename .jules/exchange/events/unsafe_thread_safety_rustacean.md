---
label: "refacts"
created_at: "2024-03-31"
author_role: "rustacean"
confidence: "high"
---

## Problem

Global state modifications like `env::set_current_dir` and `env::set_var` are used in test helpers (e.g. `DirGuard` and `PathGuard`), which alters the current process state and can cause data races across tests. The code properly uses `unsafe` for `env::set_var`, but misses wrapping `env::set_current_dir` with `unsafe` and `#[allow(unused_unsafe)]`, violating thread safety expectations for concurrent tests.

## Goal

Ensure functions modifying global process state in tests (like `env::set_current_dir` in `DirGuard` and `env::set_var` in `PathGuard`) are explicitly wrapped in `unsafe { ... }` blocks and annotated with `#[allow(unused_unsafe)]` to emphasize thread-unsafe nature, per the Testing Rule (Thread Safety Marking).

## Context

Modifying global state in Rust is intrinsically unsafe in a multi-threaded context like `cargo test`. Memory specifically dictates: "When using functions that modify global process state (e.g., `env::set_current_dir`) in tests, explicitly wrap them in an `unsafe { ... }` block and annotate with `#[allow(unused_unsafe)]` to emphasize their thread-unsafe nature...". `PathGuard` and `DirGuard` are used in parallel test runs unless serialized.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 15, 23"
  note: "`env::set_current_dir` is used without `unsafe` blocks or `#[allow(unused_unsafe)]`"
- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 47, 56, 61"
  note: "`env::set_var` and `env::remove_var` are used with `unsafe` but missing `#[allow(unused_unsafe)]` annotations"

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
