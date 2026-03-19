---
label: "refacts"
created_at: "2024-05-18"
author_role: "data_arch"
confidence: "high"
---

## Problem

Implicit validation for `BackupTarget` occurs via unwrapping `Option` at the CLI boundary, whereas `Profile` uses explicit validation functions returning a `Result`.

## Goal

Normalize validation logic across the codebase to return a typed `AppError` at the domain boundary instead of relying on CLI modules to construct domain errors.

## Context

`BackupTarget::from_input` returns an `Option<Self>`. The caller in `src/app/commands/backup/mod.rs` immediately unwraps this and manually constructs an `AppError::Backup`. In contrast, `Profile` provides explicit validation methods (`validate_profile`, `validate_machine_profile`) that directly return `Result<Profile, AppError>`. This inconsistency indicates a missing clear boundary for validation responsibilities and allows domain concepts (validation failure messages) to leak into the command orchestration logic.

## Evidence

- path: "src/domain/backup_target.rs"
  loc: "14-20"
  note: "Returns `Option<BackupTarget>` instead of returning an explicit validation error like `AppError::InvalidBackupTarget`."

- path: "src/app/commands/backup/mod.rs"
  loc: "35-40"
  note: "CLI command unwraps the option and manually constructs an error message, leaking domain rules."

- path: "src/domain/profile.rs"
  loc: "65-83"
  note: "Correctly implements explicit validation returning `Result<Profile, AppError>` at the domain level."

## Change Scope

- `src/domain/backup_target.rs`
- `src/app/commands/backup/mod.rs`
- `src/domain/error.rs`
