---
label: "tests"
created_at: "2026-03-11"
author_role: "qa"
confidence: "high"
---

## Problem

Adapter tests for Git and Jujutsu CLIs rely on the local machine state rather than isolating boundaries between pure logic and side effects.

## Goal

Decouple adapter test execution from host dependencies to prevent non-determinism and flakes across different test environments.

## Context

Adapter tests in `tests/adapters/git.rs` and `tests/adapters/jj.rs` directly invoke CLI tools like `git` and `jj`, making assumptions about the state of the host running the tests (e.g. `git config user.name`). Tests should mock or stub external system boundaries so that they can be run deterministically and safely.

## Evidence

- path: "tests/adapters/git.rs"
  loc: "14"
  note: "`get_identity` calls the real git CLI, dependent on host git config."

- path: "tests/adapters/jj.rs"
  loc: "9"
  note: "`is_available` invokes the real jj CLI and silently ignores the outcome, indicating host state pollution."

## Change Scope

- `tests/adapters/git.rs`
- `tests/adapters/jj.rs`
