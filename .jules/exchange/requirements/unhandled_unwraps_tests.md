---
label: "tests"
implementation_ready: false
---

## Goal

Ensure all test functions propagate errors correctly using `Result` and `?` rather than panicking on `unwrap()`.

## Problem

Tests in `tests/adapters/git.rs` and `tests/cli/backup.rs` extensively use `.unwrap()` for operations that could fail (like `fs::write`, `fs::set_permissions`, and `tempdir()`).

## Context

When tests fail via `.unwrap()` inside test setup logic, the resulting panic trace often obfuscates the root cause (e.g., IO error vs actual behavior failure). Returning an explicit `Result` allows the test runner to format a clearer failure message and maintains a cleaner boundary between pure logic testing and test side-effect preparation.

## Evidence

- path: "tests/adapters/git.rs"
  loc: "tests module"
  note: "Tests in `tests/adapters/git.rs` panic via `.unwrap()` on `fs::write` and `tempdir()`."
- path: "tests/cli/backup.rs"
  loc: "tests module"
  note: "Tests like `backup_system_success` use `.unwrap()` directly for `std::fs::create_dir_all`, `std::fs::write`, and `std::fs::read_to_string`."
- path: "tests/cli/switch.rs"
  loc: "tests module"
  note: "Tests like `switch_success_with_git` use `.unwrap()` directly for `std::fs::create_dir_all` and `std::fs::write`."

## Change Scope

- `tests/adapters/git.rs`
- `tests/cli/backup.rs`
- `tests/cli/switch.rs`

## Constraints

- Test function signatures must return `Result<(), Box<dyn std::error::Error>>`.

## Acceptance Criteria

- Setup and IO functions in tests use `?` instead of `.unwrap()`.
