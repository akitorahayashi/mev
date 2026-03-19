---
label: "refacts"
created_at: "2024-03-19"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The CLI descriptions for internal tools use the term `helpers` (`Git helpers.`, `GitHub CLI helpers.`). This violates the core design principle that prohibits the use of ambiguous taxonomy terms like `helpers`.

## Goal

Refactor the CLI command descriptions to avoid the term `helpers`, replacing it with descriptive, domain-aligned language (e.g., `Git integration commands.`, `GitHub CLI integration commands.`).

## Context

Using `helpers` for subcommands obfuscates the explicit functional boundaries they represent. A clear domain vocabulary is prioritized to optimize for comprehension and refactor safety.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Uses `Git helpers.` and `GitHub CLI helpers.` for the `Git` and `Gh` internal subcommands respectively."
- path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Uses `Git helpers.` and `GitHub CLI helpers.` for the `Git` and `Gh` subcommands respectively."

## Change Scope

- `src/app/cli/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`
