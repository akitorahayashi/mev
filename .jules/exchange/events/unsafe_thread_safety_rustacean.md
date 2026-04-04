---
label: "refacts"
created_at: "2024-04-04"
author_role: "rustacean"
confidence: "high"
---

## Problem

Global process state (`std::env::set_var`) is being modified within tests. This is inherently thread-unsafe and can cause unpredictable test failures if tests run concurrently, even if marked with `#[serial]` to prevent local execution overlaps, as other system states might interact with the environment variable modifications in unexpected ways.

## Goal

Ensure that global process state modification is fully encapsulated or clearly scoped to prevent thread-safety issues during testing, adhering to the "safety contract" requirements for `unsafe` blocks.

## Context

The `unsafe` block in `test_format_string` modifies `std::env::set_var("HOME", "/mock/home")`. While it has `#[serial]` and `#[allow(unused_unsafe)]`, modifying the environment is a process-global operation that poses a risk in Rust tests if not strictly managed, as tests default to running concurrently. This requires better isolation, ideally using dependency injection or a safe abstraction instead of mutating global state.

## Evidence

- path: "src/app/commands/backup/system.rs"
  loc: "258"
  note: "Test function `test_format_string` mutates global environment `std::env::set_var` inside an `unsafe` block without strict safe encapsulation or dependency injection of the home directory path."

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "57, 69, 75"
  note: "Mocking functions modify global process state (`std::env::set_var`, `std::env::set_current_dir`) within `unsafe` blocks. Although annotated with `#[allow(unused_unsafe)]` and typically used with `#[serial]`, this modifies global environment state and could lead to flaky tests or un-encapsulated obligations for users."

## Change Scope

- `src/app/commands/backup/system.rs`
- `crates/mev-internal/src/testing/env_mock.rs`
