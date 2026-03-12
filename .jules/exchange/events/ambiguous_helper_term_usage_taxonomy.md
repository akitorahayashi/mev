---
label: "refacts"
created_at: "2026-03-12"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term "helpers" is used inconsistently across the codebase. In some places it indicates internal modules/functions, and in others it indicates CLI commands delegated to other tools.

## Goal

Remove ambiguous occurrences of "helpers" or "utils" and establish clear domain vocabulary.

## Context

The repository's AGENTS.md explicitly states: "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers." The current use of "helpers" in `mod.rs` files obscures the actual responsibility of the code.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Uses 'helpers' to describe subcommands delegating to `git` and `gh` CLIs. A more precise term like 'commands' or 'integrations' would be better."
- path: "src/app/commands/backup/mod.rs"
  loc: "line 274"
  note: "Uses '// Shared helpers' as a section comment for a function resolving paths."
- path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Repeats the same use of 'helpers' for git and gh CLI commands as the main crate."

## Change Scope

- `src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`
