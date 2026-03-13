---
label: "refacts"
created_at: "2026-03-13"
author_role: "taxonomy"
confidence: "high"
---

## Problem

Ambiguous terms like `base`, `helpers`, and `common` are used as file, module, and struct names or concepts, violating the rule: "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers."

## Goal

Rename ambiguous variables, functions, structs, and modules to specific domain terms. E.g., `Profile::Common`, `config_base()`.

## Context

Ambiguous names hide responsibilities and make discovering functionality harder. They should be replaced with names defining specific single responsibilities.

## Evidence

- path: "src/domain/profile.rs"
  loc: "Profile::Common"
  note: "Common is an ambiguous profile name; it should reflect its actual responsibility (e.g. Default or Shared)."
- path: "src/domain/backup_target.rs"
  loc: "\"common\""
  note: "Subpath uses the term 'common'."
- path: "src/adapters/identity_store/paths.rs"
  loc: "config_base"
  note: "Function named config_base() uses ambiguous term 'base'."
- path: "src/app/commands/backup/mod.rs"
  loc: "// Shared helpers"
  note: "Comments use the prohibited term 'helpers'."
- path: "src/app/cli/mod.rs"
  loc: "/// Git helpers."
  note: "Comments use the prohibited term 'helpers'."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/adapters/identity_store/paths.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/cli/mod.rs`
