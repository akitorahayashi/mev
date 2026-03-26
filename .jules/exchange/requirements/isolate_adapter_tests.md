---
label: "tests"
implementation_ready: false
---

## Goal

Ensure adapter contract tests use mock tools or properly configured local repositories, rather than unconstrained host global state.

## Problem

Adapter contract tests rely on unconstrained host global state. Tests like `tests/adapters/git.rs` and `tests/adapters/jj.rs` verify whether `git` or `jj` is available on the machine running the tests. This violates the testing rule regarding unconstrained host global state and causes unpredictable CI/local execution.

## Evidence

- source_event: "adapter_tests_rely_on_host_state_qa.md"
  path: "tests/adapters/git.rs"
  loc: "10-12"
  note: "Asserts `git.is_available()` returns true, which fails on environments where git is missing."

## Change Scope

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`

## Constraints

- Adapter contract tests must use fakes, dependency injection, or proper mock configurations.

## Acceptance Criteria

- Adapter contract tests no longer rely on `git` or `jj` being globally installed on the host machine.