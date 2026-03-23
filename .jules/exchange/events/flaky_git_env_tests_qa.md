---
label: "bugs"
created_at: "2026-03-23"
author_role: "qa"
confidence: "high"
---

## Problem

Tests in `crates/mev-internal/src/adapters/git.rs` and `gh.rs` use `std::env::set_var("PATH", ...)` to inject mock external binaries. This modifies global process state and makes the test suite highly susceptible to non-determinism, data races, and flakiness when tests are executed in parallel. While `#[serial]` is used on some tests as a workaround, tests are missing isolation by design.

## Goal

Refactor the test doubles or the `process` execution boundary so that external dependencies (like the `PATH` variable or executable paths) are passed explicitly via dependency injection, avoiding global state manipulation.

## Context

Global environment variable modification via `unsafe { env::set_var(...) }` in tests violates the "Isolation By Design" principle and causes flakiness unless all tests touching the environment are serialized, increasing the test feedback loop duration.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "55"
  note: "Modifies global process PATH using unsafe `env::set_var`."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "73"
  note: "Requires `#[serial(env_path)]` macro due to global state modification in tests."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
