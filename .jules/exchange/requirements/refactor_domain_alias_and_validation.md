---
label: "refacts"
implementation_ready: false
---

## Goal

Consolidate alias resolution logic and explicit boundary validation within domain models (`BackupTarget`, `Profile`, `SwitchIdentity`) to prevent redundant implementations, eliminate domain rules leaking into CLI logic, and ease schema evolution.

## Problem

Duplicate alias resolution logic exists across multiple domain entities (`Profile`, `SwitchIdentity`, and `BackupTarget`), which implement custom, slightly different string-to-enum parsing logic. In addition, implicit validation for `BackupTarget` occurs via unwrapping `Option` at the CLI boundary, whereas `Profile` uses explicit validation functions returning a `Result`. This inconsistency allows domain concepts to leak into the command orchestration logic. Lastly, adding a new backup target requires changing multiple disparate locations in `BackupTarget`, which violates the Single Source of Truth principle and is prone to errors during schema evolution.

## Evidence

- source_event: "duplicate_alias_resolution_logic_data_arch.md"
  path: "src/domain/vcs_identity.rs"
  loc: "44-60"
  note: "Implements custom loop over `SWITCH_IDENTITY_ALIASES` to resolve input strings."
- source_event: "duplicate_alias_resolution_logic_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "55-63"
  note: "Implements custom loop over `PROFILE_ALIASES` to resolve input strings."
- source_event: "duplicate_alias_resolution_logic_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "14-20"
  note: "Implements custom match expression mapping inputs to `BackupTarget` variants."
- source_event: "missing_explicit_validation_boundary_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "14-20"
  note: "Returns `Option<BackupTarget>` instead of returning an explicit validation error like `AppError::InvalidBackupTarget`."
- source_event: "missing_explicit_validation_boundary_data_arch.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "35-40"
  note: "CLI command unwraps the option and manually constructs an error message, leaking domain rules."
- source_event: "missing_explicit_validation_boundary_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "65-83"
  note: "Correctly implements explicit validation returning `Result<Profile, AppError>` at the domain level."
- source_event: "schema_evolution_concern_in_backup_target_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "14-49"
  note: "Adding a new target requires changes to `from_input`, `all()`, `name()`, `description()`, and `role()`."
- source_event: "schema_evolution_concern_in_backup_target_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "22-25"
  note: "Hardcoded array in `all()` requires manual maintenance and is out of sync with the enum definition."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/domain/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/domain/error.rs`

## Constraints

- Ensure all alias logic resolves in one place.
- All boundaries must explicitly fail and return an `AppError` on invalid mappings.

## Acceptance Criteria

- Alias resolution is centralized and uniformly utilized.
- `BackupTarget` implements explicit boundaries without Option unwrapping in the CLI.
- Changes to `BackupTarget` arrays and switch cases are consolidated so new entries are easier to configure.
