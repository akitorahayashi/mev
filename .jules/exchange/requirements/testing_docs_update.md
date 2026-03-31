---
label: "docs"
implementation_ready: false
---

## Goal

Add `tests/cli/` and `tests/adapters/` to the table of layers and the command list in `docs/testing.md`.

## Problem

The testing documentation in `docs/testing.md` fails to mention the `tests/cli/` and `tests/adapters/` test layers, and consequently doesn't include the command to run those tests.

## Context

The `tests/` directory contains several integration testing targets including `cli`, `library`, `runtime`, `security`, and `adapters`. The `docs/architecture.md` file correctly outlines the structure (e.g., `cli.rs + cli/ # CLI behavior contracts`, `adapters.rs + adapters/ # Adapter behavior contracts`). However, `docs/testing.md` only documents `library`, `runtime`, and `security` tests, entirely omitting `cli` and `adapters` in both the "Testing is organized into distinct layers..." table and the "Run specific test categories:" section.

## Evidence

- path: "docs/testing.md"
  loc: "7-12"
  note: "The `tests/cli/` and `tests/adapters/` layers are missing from the table mapping layers to locations."
- path: "docs/testing.md"
  loc: "33-36"
  note: "The commands to run cli and adapter tests (`cargo test --test cli`, `cargo test --test adapters`) are missing from the list of specific test categories."
- path: "docs/architecture.md"
  loc: "58-60"
  note: "This file correctly documents the existence and purpose of `cli.rs + cli/` and `adapters.rs + adapters/`."

## Change Scope

- `docs/testing.md`

## Constraints

- Keep it consistent with `docs/architecture.md` and actual repository structure.

## Acceptance Criteria

- `tests/cli/` and `tests/adapters/` are documented in `docs/testing.md`.
