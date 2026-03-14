---
label: "refacts"
created_at: "2026-03-14"
author_role: "data_arch"
confidence: "high"
---

## Problem

Core domain models contain CLI-specific string input parsing logic and aliases, violating Boundary Sovereignty and I/O decoupling rules.

## Goal

Move CLI input mapping, alias resolution, and string parsing to the adapter or application CLI layer, keeping domain models pure and independent of transport concerns.

## Context

The architecture rules mandate that domain pure logic ports abstract file system concepts and I/O away. Specifically, domain input parsing must not contain CLI-specific string input aliases. Validation and UI mapping should exclusively be handled by the adapter or application CLI layer. Currently, domain models like `SwitchIdentity`, `Profile`, and `BackupTarget` directly handle string resolution and CLI aliases.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "SWITCH_IDENTITY_ALIASES"
  note: "Defines CLI aliases for identity resolution directly in the domain model."
- path: "src/domain/vcs_identity.rs"
  loc: "resolve_switch_identity"
  note: "Implements string-to-enum resolution using CLI aliases."
- path: "src/domain/profile.rs"
  loc: "PROFILE_ALIASES"
  note: "Defines CLI aliases for profile resolution in the domain layer."
- path: "src/domain/backup_target.rs"
  loc: "BackupTarget::from_input"
  note: "Implements CLI input parsing directly within the domain model."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/switch.rs`
- `src/app/cli/make.rs`
- `src/app/cli/create.rs`
- `src/app/cli/backup.rs`
