---
label: "tests"
created_at: "2024-05-30"
author_role: "qa"
confidence: "high"
---

## Problem

Adapter contract tests rely on unconstrained host global state. Tests like `tests/adapters/git.rs` and `tests/adapters/jj.rs` verify whether `git` or `jj` is available on the machine running the tests (`is_available`). If the test environment doesn't have these installed, `which::which("git")` or `which::which("jj")` fails, leading to differences in test execution. For `jj_cli_is_available_returns_bool`, it only checks that it doesn't panic. For `git_cli_reports_available`, it strictly asserts `assert!(git.is_available());`, causing an actual test failure if `git` is missing.

## Goal

Ensure adapter contract tests use mock tools, fakes, dependency injection, or properly configured local repositories rather than unconstrained global host state so they run deterministically and correctly on all environments.

## Context

Testing against host environments is an anti-pattern. Adapter contract tests should rely on test fakes or properly constructed mock directories where tools like `git` and `jj` are isolated from the host environment to guarantee reproducibility across all developer and CI machines. Relying on `git` or `jj` binaries in the system's `$PATH` directly violates the testing rule regarding unconstrained host global state.

## Evidence

- path: "tests/adapters/git.rs"
  loc: "10-12"
  note: "Asserts `git.is_available()` returns true, which fails on environments where git is missing."
- path: "tests/adapters/jj.rs"
  loc: "10-13"
  note: "Test ignores the `is_available()` result because it acknowledges `jj` might be missing in CI. Relies on host state."

## Change Scope

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`
