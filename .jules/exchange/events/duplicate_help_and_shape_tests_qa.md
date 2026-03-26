---
label: "tests"
created_at: "2024-05-30"
author_role: "qa"
confidence: "high"
---

## Problem

Duplicate boundary test coverage for CLI argument checks. Each subcommand test file verifies `--help` output attributes, instead of concentrating global argument and general shape tests in a single place.

## Goal

Consolidate basic CLI shape checks (like validating `--help` outputs, presence of standard flags, handling of unrecognised commands) into a central boundary test file (e.g., `tests/cli/help_and_version.rs`) to prevent test duplication and reduce redundant maintenance across many files.

## Context

Running similar shape tests repeatedly across `tests/cli/list.rs`, `tests/cli/make.rs`, `tests/cli/create.rs`, and others adds to the total execution time, makes modifying help text difficult, and fails to follow the strategy of testing behavior rather than internals or redundant properties across multiple modules. The testing rules explicitly dictate consolidating CLI shape checks into a single boundary test.

## Evidence

- path: "tests/cli/make.rs"
  loc: "6-24"
  note: "Verifies make help shows overwrite flag, verbose flag, and profile flag."
- path: "tests/cli/create.rs"
  loc: "7-25"
  note: "Verifies create help shows overwrite flag, verbose flag."
- path: "tests/cli/list.rs"
  loc: "6-17"
  note: "Verifies list help shows description and alias."
- path: "tests/cli/backup.rs"
  loc: "6-14"
  note: "Verifies backup help shows target argument."

## Change Scope

- `tests/cli/help_and_version.rs`
- `tests/cli/create.rs`
- `tests/cli/make.rs`
- `tests/cli/list.rs`
- `tests/cli/backup.rs`
- `tests/cli/switch.rs`
- `tests/cli/config.rs`
- `tests/cli/identity.rs`
