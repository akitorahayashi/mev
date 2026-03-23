---
label: "tests"
created_at: "2024-03-23"
author_role: "cov"
confidence: "high"
---

## Problem

The system and VSCode settings backup orchestration (`src/app/commands/backup/mod.rs`) suffers from extremely low coverage (9 out of 149 lines covered), failing to validate crucial fallback values and file writes.

## Goal

Ensure that the fallback and normalization logic for system default backups and VSCode extension list parsing are robustly tested to avoid silent failures or data loss when configurations shift.

## Context

According to `cargo tarpaulin` coverage reports, the logic inside `src/app/commands/backup/mod.rs` is responsible for parsing configuration definitions, querying macOS defaults, validating settings, and generating YAML/JSON artifacts. This represents a complex and highly failure-prone data translation boundary. Without significant test coverage, new features or refactorings risk breaking user backup states.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "59-247"
  note: "Only 9/149 lines covered. Critical methods like `execute_system`, `format_value`, and `execute_vscode` lack line-level tests, risking regressions when data formats change."

## Change Scope

- `src/app/commands/backup/mod.rs`
- `tests/cli_contracts/backup.rs` (to be created or modified)