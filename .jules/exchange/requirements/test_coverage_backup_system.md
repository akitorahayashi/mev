---
label: "tests"
implementation_ready: true
---

## Goal

Add tests to ensure all `format_bool`, `format_numeric`, `format_string` fallback/default logic paths are verified, as well as the missing coverage on directory fallback orchestration.

## Problem

The system backup logic in `src/app/commands/backup/system.rs` lacks coverage on edge cases related to formatting logic and default handling, increasing regression risk.

## Context

The system backup functionality evaluates macOS system settings against defined definition files. Value parsing handles coercions (e.g., bool parsing, float/int parsing, and home directory replacement strings). The tarpaulin coverage report (default configuration, line coverage metric) shows significant uncovered logic paths when mapping default states, executing the file writing sequence, or parsing truthy values. These data formatting decisions are critical as a failure here results in a corrupted, unusable backup configuration.

## Evidence

- path: "src/app/commands/backup/system.rs"
  loc: "execute, format_bool, format_numeric, format_string"
  note: "The missed regions comprise nearly the entirety of the format_* mapping logic which handles critical coercion paths. We need to test the file execution generation as well as value formatting."

## Change Scope

- `src/app/commands/backup/system.rs`

## Constraints

- Ensure all formatting logic paths are tested.

## Acceptance Criteria

- High test coverage is achieved for system backup formatting and execution logic.
