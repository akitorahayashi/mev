---
label: "refacts"
implementation_ready: false
---

## Goal

Delegate all local file system mutations to `FsPort` to ensure I/O constraints are decoupled from the application logic orchestration and to permit injecting test doubles for unit-testing.

## Problem

The application layer performs side-effecting I/O directly via `std::fs` operations, bypassing the port/adapter boundary (`FsPort`).

## Context

Per the "Architecture Rule (Application I/O Side Effects)", application orchestration commands must not bypass the port/adapter boundary. Direct usage of `std::fs` and `std::path` creates a hidden coupling and prevents proper dependency injection in test suites.

## Evidence

- source_event: "application_io_side_effects_structural_arch.md"
  path: "src/app/commands/deploy_configs.rs"
  loc: "42-74"
  note: "Directly uses `std::fs::remove_dir_all`, `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."
- source_event: "application_io_side_effects_structural_arch.md"
  path: "src/app/commands/config/mod.rs"
  loc: "43-71"
  note: "Directly uses `std::fs::remove_dir_all`, `std::fs::rename`, `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."

## Change Scope

- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
