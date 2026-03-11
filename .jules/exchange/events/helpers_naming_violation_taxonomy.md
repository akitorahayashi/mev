---
label: "refacts"
created_at: "2026-03-11"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term "helpers" is used to describe CLI subcommands and internal functions, violating the design rule against ambiguous names or responsibilities.

## Goal

Rename "helpers" commands and functions to specific, domain-aligned nouns or verbs that describe their exact responsibility.

## Context

The repository enforces a design rule: "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers."
The term "helpers" obscures the actual function being performed. In the CLI, it hides the true capabilities of `git` and `gh` integrations behind a generic bucket. In internal code, it acts as a catch-all for logic that hasn't been properly assigned to a single, responsible component.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "67"
  note: "Uses 'Git helpers' as the help string for the `Git` subcommand, providing no information about what the command actually does."

- path: "src/app/cli/mod.rs"
  loc: "71"
  note: "Uses 'GitHub CLI helpers' as the help string for the `Gh` subcommand."

- path: "src/app/commands/backup/mod.rs"
  loc: "274"
  note: "Uses '// Shared helpers' as a comment block to group functions, avoiding the assignment of those functions to specific responsibilities."

## Change Scope

- `src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`
