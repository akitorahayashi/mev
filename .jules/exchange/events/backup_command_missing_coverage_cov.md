---
label: "tests"
created_at: "2026-03-14"
author_role: "cov"
confidence: "high"
---

## Problem

The `backup` command logic in `src/app/commands/backup/mod.rs` has extremely low coverage (13/153 lines, 8.5%).

## Goal

Increase coverage for the `backup` command to control the risk of regressions in backup generation, file writing, and system settings resolution.

## Context

The `backup` command is responsible for reading macOS system defaults and VSCode extensions, processing them, and writing configuration YAML/JSON files safely to disk. Since this involves interacting with external ports (`FsPort`, `MacosDefaultsPort`, `VscodePort`) and manipulating user data, regressions could lead to silent data loss or corrupted backup files. Currently, the test suite only covers the CLI contract and argument parsing for `backup`, not the actual orchestration and logic.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "44-310"
  note: "Line coverage is at 13/153 lines (8.5%). The untested lines encompass the entire logic for `execute_system` and `execute_vscode`, which dictate how backups are processed and saved."

## Change Scope

- `src/app/commands/backup/mod.rs`
- `tests/cli/backup.rs`
