---
label: "refacts"
created_at: "2026-03-14"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `backup` command emits a diagnostic warning about missing local definitions directly to `stdout` instead of `stderr`.

## Goal

Route the missing local definitions fallback warning to `stderr` to preserve strict I/O separation.

## Context

The `cli_sentinel` contract dictates strict I/O separation: "stdout carries result data; stderr carries warnings, logs, and errors. Mixed streams break automation." The fallback to package defaults is a warning state and should not pollute standard output streams.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "57-61"
  note: "Uses `println!` to emit 'Local definitions not found at ... Using package defaults.' instead of `eprintln!`."

## Change Scope

- `src/app/commands/backup/mod.rs`
