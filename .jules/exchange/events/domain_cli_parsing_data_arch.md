---
label: "refacts"
created_at: "2024-05-24"
author_role: "data_arch"
confidence: "high"
---

## Problem

Core domain models (`Profile`, `SwitchIdentity`, `BackupTarget`) contain CLI-specific string input parsing logic, aliases, and validation.

## Goal

Remove CLI parsing concerns from core domain models, moving validation and input resolution to the adapter or application CLI layer.

## Context

Domain models represent the pure, canonical state and rules of the application. By embedding CLI aliases (e.g., "mbk" for Macbook) and string parsing (`from_input`, `resolve_switch_identity`) directly into the domain types, UI/transport concerns are leaking into the core domain logic, violating Boundary Sovereignty.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "resolve_switch_identity"
  note: "`resolve_switch_identity` handle CLI input mapping in the domain."
- path: "src/domain/profile.rs"
  loc: "resolve_profile, validate_machine_profile, validate_profile"
  note: "`resolve_profile` and `validate_machine_profile` embed CLI string mapping and validation logic."
- path: "src/domain/backup_target.rs"
  loc: "from_input"
  note: "`from_input` parses user-provided strings, including an alias 'vscode-extensions'."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/`
