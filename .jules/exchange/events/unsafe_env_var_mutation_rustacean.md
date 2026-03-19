---
label: "refacts"
created_at: "2024-05-23"
author_role: "rustacean"
confidence: "high"
---

## Problem

The `EnvGuard` struct in `src/adapters/ansible/executor.rs` uses `unsafe { env::set_var(...) }` and `unsafe { env::remove_var(...) }` to mutate process-wide environment variables during unit tests.

## Goal

Remove `unsafe` environment variable mutations from the test suite to prevent undefined behavior and data races. Use `serial_test` with a safer approach or redesign the test to avoid process-global state changes, potentially by passing environment variables explicitly through a context or using `std::process::Command::env()` and testing the command builder instead of relying on global env state for binary resolution tests.

## Context

Modifying process environment variables (`env::set_var`, `env::remove_var`) is inherently unsafe in Rust when multiple threads are running, as it can cause data races with any other code reading the environment. While the tests are marked with `#[serial]`, using `unsafe` blocks for this is an anti-pattern and often masks a design issue where global state is used instead of explicit dependency injection or configuration objects.

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "304, 311, 319, 321"
  note: "`EnvGuard` uses `unsafe { env::set_var(...) }` and `unsafe { env::remove_var(...) }`."

## Change Scope

- `src/adapters/ansible/executor.rs`
