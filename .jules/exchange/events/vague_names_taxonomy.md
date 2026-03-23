---
label: "refacts"
created_at: "2024-05-18"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The codebase uses vague and prohibited names ("helpers") which violates the taxonomy and naming rules outlined in the architecture and AGENTS.md documentation.

## Goal

Refactor the codebase to use more specific and domain-appropriate terminology rather than falling back on ambiguous catch-all terms like "helpers".

## Context

The repository's architecture documentation explicitly forbids ambiguous names such as `core/`, `utils/`, and `helpers/` to ensure every file and module belongs to a clear, specific category. However, several modules still use the term "helpers" in comments and documentation to describe utility functions.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "line 270"
  note: "Uses the term `// Shared helpers` as a section divider for utility functions."
- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 1"
  note: "Module documentation uses `//! Test helpers for mocking the environment.`"
- path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Subcommand documentation describes Git and GitHub commands as `/// Git helpers.` and `/// GitHub CLI helpers.`"
- path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Subcommand documentation describes Git and GitHub commands as `/// Git helpers.` and `/// GitHub CLI helpers.`"

## Change Scope

- `src/app/commands/backup/mod.rs`
- `crates/mev-internal/src/testing/env_mock.rs`
- `src/app/cli/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`
