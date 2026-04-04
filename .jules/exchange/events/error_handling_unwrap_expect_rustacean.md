---
label: "refacts"
created_at: "2024-04-04"
author_role: "rustacean"
confidence: "high"
---

## Problem

Widespread use of `unwrap()` and `expect()` bypasses structured error handling and poses a panicking risk, especially in testing and domain models where errors should be clearly propagated and typed.

## Goal

Replace usages of `unwrap()` and `expect()` with explicit error propagation using `Result<(), Box<dyn std::error::Error>>` (for tests) or custom typed errors (for domain types) to adhere to the explicit error handling principles.

## Context

Using `unwrap()` and `expect()` circumvents Rust's type-safe error boundaries. Even in tests, failures should be structured and propagated rather than panicking, which can interrupt test execution and obscure diagnosis. In `mev-internal/src/domain/repository_ref.rs`, `expect()` is used extensively to bypass error handling for `from_repo_arg` and remote parsing tests.

## Evidence

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "44, 46, 12, 14, 15, 84"
  note: "Uses `unwrap()` and `expect()` heavily for file system and environment manipulation during testing."

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "42-53"
  note: "Tests use `.expect()` extensively instead of explicitly propagating errors and using `?`."

- path: "src/testing/fs.rs"
  loc: "58, 62"
  note: "Uses `.unwrap()` to strip path prefixes."

## Change Scope

- `crates/mev-internal/src/testing/env_mock.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `src/testing/fs.rs`
