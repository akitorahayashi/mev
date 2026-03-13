---
label: "tests"
created_at: "2024-05-15"
author_role: "qa"
confidence: "high"
---

## Problem

The CLI contract tests instantiate multiple `TestContext` structs implicitly via `TestContext::new()`. During cargo's parallel test execution, multiple `TestContext::new()` calls map to parallel operations that set environment variables (like `HOME`). The Rust testing harness for modifying global environment variables without serial execution leads to race conditions, flakiness, or test failures across independent test runs.

## Goal

Ensure all integration tests that rely on shared environmental context state overrides are isolated explicitly and run serially using the `serial_test` crate.

## Context

Running integration and cli contract tests concurrently through Rust's default test harness means multiple processes/threads can interfere with each other if they are sharing mutable states such as test working directories and overriding standard environment variables `HOME`. This is an anti-pattern as described in the testing principles regarding isolating tests by design and preventing flakes at the source.

## Evidence

- path: "tests/harness/test_context.rs"
  loc: "TestContext::cli"
  note: "This sets the HOME environment variable for the testing command, which affects integration tests that depend on checking global env configurations."
- path: "tests/cli/switch.rs"
  loc: "switch_help_shows_identity_argument"
  note: "Like many other CLI tests, it instantiates `TestContext::new()` and runs concurrently with other tests."

## Change Scope

- `tests/cli/*.rs`
- `tests/harness/test_context.rs`