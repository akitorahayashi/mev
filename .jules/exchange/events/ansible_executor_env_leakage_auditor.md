---
label: "tests"
created_at: "2024-04-17"
author_role: "auditor"
confidence: "medium"
---

## Problem

Tests in `src/adapters/ansible/executor.rs` and `tests/adapters/version.rs` (and potentially others) might rely on global environment variables rather than explicitly mocked context, although `env_var` injection is used in some places. Specifically, `std::env::var("PATH")` is read in `version_source.rs` which is tested in `tests/adapters/version.rs`.

## Goal

Ensure all environment variable interactions are fully controlled and mocked in tests, avoiding global state leakage that causes non-determinism.

## Context

Depending on global system state (like `$PATH` or `$HOME`) within unit tests introduces flakiness and order-dependency. When global state changes (e.g. parallel tests, different CI environments), these tests can fail unpredictably.

## Evidence

- path: "src/adapters/version_source.rs"
  loc: "22"
  note: "Directly reads system PATH without dependency injection, making tests relying on it non-deterministic across different host environments."

## Change Scope

- `src/adapters/version_source.rs`
- `tests/adapters/version.rs`
