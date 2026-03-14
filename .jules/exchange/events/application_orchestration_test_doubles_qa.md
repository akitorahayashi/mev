---
label: "tests"
created_at: "2026-03-14"
author_role: "qa"
confidence: "high"
---

## Problem

The application orchestration layer (`src/app/commands/`) cannot be unit tested in isolation using test doubles because it depends on a concrete `DependencyContainer` rather than port interfaces. This forces integration tests to assert on partial failures (e.g., missing assets) rather than verifying pure command logic.

## Goal

Refactor command dependencies to use port interfaces (e.g., `&dyn AnsiblePort`) so that orchestration logic can be validated with fast, in-process test doubles instead of slow, brittle binary invocations.

## Context

Testing pure logic with slow integration/E2E tests that execute the full binary is an anti-pattern. This prevents testing application command boundaries without side effects. Currently, integration tests like `tests/cli/backup.rs` test partial success (subcommand parsing) followed by failure (missing assets), which is a brittle test structure that asserts error behavior instead of positive logic flow.

## Evidence

- path: "src/app/commands/create/mod.rs"
  loc: "14"
  note: "`pub fn execute(ctx: &DependencyContainer, ...)` tightly couples logic to concrete adapters."
- path: "tests/cli/backup.rs"
  loc: "12-14"
  note: "`backup_alias_bk_is_accepted` explicitly expects failure due to missing ansible assets but tests alias resolution, demonstrating a lack of isolated environment testing."
- path: "src/testing/mod.rs"
  loc: "1"
  note: "`//! In-process test doubles and builders.` module is empty, indicating missing testing infrastructure for this architectural boundary."

## Change Scope

- `src/app/commands/`
- `src/testing/`
- `tests/cli/`
