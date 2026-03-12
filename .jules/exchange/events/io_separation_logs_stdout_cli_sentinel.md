---
label: "refacts"
created_at: "2023-10-25"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

Human-readable logs, progress indicators, and informative messages are emitted to `stdout` rather than `stderr` across several key CLI commands (`create`, `make`, `switch`, `backup`). This mixes diagnostic or informative output with actual result data, potentially breaking automation that relies on parsing `stdout` for results.

## Goal

Ensure all human-readable diagnostics, logs, and progress output use `stderr` (`eprintln!`), reserving `stdout` exclusively for structured, script-parseable result data (or keeping it completely empty).

## Context

Mixing logs and progress output in `stdout` prevents commands from being safely piped to other tools (like `jq` or `grep`). Automation requires a strict separation where `stdout` is the true payload, while `stderr` handles human feedback. Output streams must not be ambiguous.

## Evidence

- path: "src/app/commands/create/mod.rs"
  loc: "println!(\"mev: Creating {} environment\", plan.profile);"
  note: "Outputs raw log text to `stdout` during environment creation."
- path: "src/app/commands/switch/mod.rs"
  loc: "println!(\"Switching to {} identity...\", identity);"
  note: "Outputs state changes and progress logs to `stdout`."
- path: "src/app/commands/backup/mod.rs"
  loc: "println!(\"Running backup: {}\", target.description());"
  note: "Informational message about running backups sent to `stdout`."
- path: "src/app/commands/make/mod.rs"
  loc: "println!(\"Running tags: {}\", plan.tags.join(\", \"));"
  note: "Diagnostic progress information sent to `stdout`."

## Change Scope

- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/commands/deploy_configs.rs`
