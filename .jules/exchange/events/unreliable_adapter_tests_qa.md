---
label: "tests"
created_at: "2026-03-23"
author_role: "qa"
confidence: "high"
---

## Problem

Adapter contract tests (`tests/adapters/git.rs` and `tests/adapters/jj.rs`) depend entirely on the host environment having configured external CLIs like `git` or `jj`. They assert `is_available()` blindly, or in `get_identity()`, they assert `result.is_ok()` which will falsely fail in sandboxed or new CI environments where a global git identity isn't configured.

## Goal

Adapter tests should not couple to unconstrained host global state. Tests need to be made deterministic: either inject a test fake for the underlying process runner, or explicitly prepare a mock directory structure (like a local git repo) with known config variables before asserting on git outputs.

## Context

Running `assert!(result.is_ok())` on a system-level command like `git config user.name` without setting up a sandbox repository makes the test flakiness completely out of the developer's control. It breaks the "Determinism Over Retries" principle by relying on whatever the developer's machine happens to have configured globally.

## Evidence

- path: "tests/adapters/git.rs"
  loc: "12"
  note: "Asserts `git.get_identity()` is always OK, which fails if the host has no git user configured."
- path: "tests/adapters/jj.rs"
  loc: "9"
  note: "Just verifies no panic, failing to assert any meaningful behavioral property."

## Change Scope

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`
