---
label: "tests"
created_at: "2024-03-31"
author_role: "qa"
confidence: "high"
---

## Problem

Tests in `tests/adapters/git.rs` and `tests/cli/backup.rs` extensively use `.unwrap()` for operations that could fail (like `fs::write`, `fs::set_permissions`, and `tempdir()`), violating the testing error handling rule. Tests should return `Result<(), Box<dyn std::error::Error>>` and explicitly propagate errors with the `?` operator.

## Goal

Ensure all test functions propagate errors correctly using `Result` and `?` rather than panicking on `unwrap()`, improving diagnosability when tests fail due to setup or IO issues.

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
