---
label: "refacts"
created_at: "2024-03-19"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The internal documentation comment `// Shared helpers` in `src/app/commands/backup/mod.rs` uses the forbidden term `helpers`, which violates the architectural constraint: "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers."

## Goal

Remove the ambiguous taxonomy (`Shared helpers`) from the codebase, replacing it with a precise, domain-aligned term that reflects the function's responsibility (e.g., `Resolution logic`).

## Context

The `AGENTS.md` and repository guidelines strictly prohibit ambiguous taxonomy like `base`, `common`, `core`, `utils`, or `helpers`. The presence of `Shared helpers` as a section comment within a module violates these constraints. Renaming it to clarify the exact purpose improves comprehensibility and adherence to the guidelines.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "line 275"
  note: "The comment reads `// Shared helpers` which explicitly uses the forbidden term `helpers`."

## Change Scope

- `src/app/commands/backup/mod.rs`
