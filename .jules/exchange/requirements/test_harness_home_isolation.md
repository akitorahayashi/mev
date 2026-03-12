---
label: "tests"
implementation_ready: true
---

## Goal

Ensure complete environment isolation for CLI integration tests by overriding the `HOME` environment variable to a temporary directory.

## Problem

The `TestContext` integration test harness does not override the `HOME` environment variable, which allows tests to accidentally read/write the host user's actual configuration (e.g., `~/.config/mev/identity.json`). This violates test isolation and can corrupt developer configurations.

## Evidence

- source_event: "global_state_leak_integration_tests_qa.md"
  path: "tests/harness/test_context.rs"
  loc: "line 29-34"
  note: "`TestContext::cli()` sets the current working directory via `cmd.current_dir(&self.work_dir)`, but does not override `HOME` using `cmd.env(\"HOME\", &self.work_dir)`."

## Change Scope

- `tests/harness/test_context.rs`

## Constraints

- Any overridden variables must not disrupt standard test execution environments like GitHub Actions.
- Consider utilizing `serial_test` if race conditions appear during Cargo's test parallelization.

## Acceptance Criteria

- The test harness reliably sets `HOME` to the temporary workspace directory during command execution.
- Tests do not interact with the host machine's configuration directory.
