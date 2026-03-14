---
label: "refacts"
created_at: "2025-03-14"
author_role: "rustacean"
confidence: "high"
---

## Problem

Use of `.unwrap_or()` for plausible runtime failures or parsing masks the actual failure, causing silent fallback behaviors instead of surfacing explicit errors.

## Goal

Replace silent fallbacks with explicit error propagation or typed boundary validation to prevent masking critical application or execution failures.

## Context

The Rustacean design rules prohibit silent fallbacks (`unwrap_or`) on production paths where failure is plausible. Masking failures makes debugging difficult and allows configuration or state to drift without user awareness.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "204, 208"
  note: "Parsing floats and formatting strings falls back to the original string or original value using `.unwrap_or()`, silently masking parse errors."
- path: "src/adapters/ansible/executor.rs"
  loc: "152"
  note: "The `ansible-playbook` exit code is fetched using `.unwrap_or(-1)` if `code()` is None (e.g., killed by signal). This masks signal termination as an arbitrary exit code."
- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "run_status"
  note: "Process exit code is silently defaulted to 1 via `unwrap_or(1)` if terminated by a signal."

## Change Scope

- `src/app/commands/backup/mod.rs`
- `src/adapters/ansible/executor.rs`
- `crates/mev-internal/src/adapters/process.rs`
