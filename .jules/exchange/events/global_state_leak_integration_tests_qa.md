---
label: "tests"
created_at: "2025-01-14"
author_role: "qa"
confidence: "high"
---

## Problem

The integration testing harness (`TestContext`) isolates the current working directory but fails to isolate global user state (the `HOME` environment variable). This allows CLI tests to inadvertently read from or write to the host developer's actual configuration (`~/.config/mev/identity.json`).

## Goal

Ensure complete environment isolation for CLI integration tests by overriding `HOME` to a temporary directory within the test harness, preventing accidental host state mutation and guaranteeing deterministic test runs regardless of the developer's local setup.

## Context

Tests are developer tooling that must be deterministically safe. Because `TestContext::cli()` does not overwrite `HOME`, commands like `mev identity set` or `mev switch` executed during a test run will reach out to `dirs::home_dir()` (the actual host machine's home directory) via the `IdentityFileStore`. This violates the "Isolation By Design" principle by introducing shared mutable state between the host machine and the test runner.

## Evidence

- path: "tests/harness/test_context.rs"
  loc: "line 29-34"
  note: "`TestContext::cli()` sets the current working directory via `cmd.current_dir(&self.work_dir)`, but does not override `HOME` using `cmd.env(\"HOME\", &self.work_dir)`."

## Change Scope

- `tests/harness/test_context.rs`