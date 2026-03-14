---
label: "docs"
created_at: "2026-03-14"
author_role: "taxonomy"
confidence: "high"
---

## Problem
The term `helpers` is used generically in module documentation and comments without specifying the domain noun or responsibility, violating the anti-pattern against vague names.

## Goal
Replace occurrences of the generic term "helpers" with precise, domain-oriented descriptions that specify the logic's responsibility (e.g., "Backup definitions resolution" or "Git CLI commands").

## Context
Generic names like "helpers" or "utils" obscure the true purpose of the code. Code should describe what it does and why, not that it "helps".

## Evidence
- path: "src/app/commands/backup/mod.rs"
  loc: "// Shared helpers"
  note: "Contains the generic comment `// Shared helpers` instead of describing the domain logic (e.g., definitions resolution)."
- path: "src/app/cli/mod.rs"
  loc: "/// Git helpers."
  note: "Describes internal git commands generically as `/// Git helpers.`"

## Change Scope
- `src/app/commands/backup/mod.rs`
- `src/app/cli/mod.rs`
