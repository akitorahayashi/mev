---
label: "bugs"
created_at: "2026-03-11"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

Operational logs, progress updates, and success messages are written to `stdout` instead of `stderr`, mixing diagnostic information with result data.

## Goal

Migrate all operational logs, progress messages, and non-result output to `stderr` (e.g., using `eprintln!`), reserving `stdout` strictly for command result data.

## Context

The `cli_sentinel` contract mandates strict I/O separation: `stdout` must carry only result data, while `stderr` carries warnings, logs, and errors. Currently, commands like `create`, `make`, `backup`, and `update` use `println!` for logging progress and status updates. Mixed streams cause automation failures, as piped output will include non-data logs rather than pure result streams.

## Evidence

- path: "src/app/commands/create/mod.rs"
  loc: "println!(\"[{step}/{total}] Running: {tag}\");"
  note: "Progress logs are emitted to stdout."
- path: "src/app/commands/make/mod.rs"
  loc: "println!(\"Running tags: {}\", plan.tags.join(\", \"));"
  note: "Operation logs are emitted to stdout."
- path: "src/app/commands/backup/mod.rs"
  loc: "println!(\"Running backup: {}\", target.description());"
  note: "Start logs are emitted to stdout."
- path: "src/app/commands/update/mod.rs"
  loc: "println!(\"Running upgrade...\");"
  note: "Process logs are emitted to stdout."
- path: "src/app/commands/deploy_configs.rs"
  loc: "println!(\"  Deployed config for {role}\");"
  note: "Deployment progress logs are emitted to stdout."

## Change Scope

- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/commands/update/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`
