---
label: "refacts"
created_at: "2026-03-23"
author_role: "qa"
confidence: "medium"
---

## Problem

Tests verifying CLI flag parsing (like `--overwrite` or `--verbose` in help text) are duplicated across multiple command tests (e.g., `tests/cli/create.rs`, `tests/cli/make.rs`). This leads to redundancy, increasing maintenance cost without adding coverage for the underlying CLI definition logic.

## Goal

Consolidate basic CLI shape checks (like help outputs for common flags) into a single boundary test (e.g., `tests/cli/help_and_version.rs` or similar) to ensure the CLI structure is validated without copy-pasting tests for every subcommand.

## Context

Testing the `clap` CLI parser's help output for global or common flags redundantly across every subcommand violates the "Recovery Cost Optimization" principle. If a global flag changes, multiple files will break simultaneously, slowing down the feedback loop.

## Evidence

- path: "tests/cli/create.rs"
  loc: "8"
  note: "Tests `create_help_shows_overwrite_flag` and `create_help_shows_verbose_flag`."
- path: "tests/cli/make.rs"
  loc: "7"
  note: "Tests `make_help_shows_overwrite_flag` and `make_help_shows_verbose_flag`."

## Change Scope

- `tests/cli/create.rs`
- `tests/cli/make.rs`
- `tests/cli/help_and_version.rs`
