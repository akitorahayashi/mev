---
label: "refacts"
created_at: "2024-04-17"
author_role: "taxonomist"
confidence: "high"
---

## Problem

The naming of enum validation and resolution functions is inconsistent across the domain layer, with some modules using `resolve_` and `validate_` prefixes for identical concepts, while their structure and return types differ slightly. Additionally, error names and messages for these resolutions drift.

## Goal

Align the function signatures and naming shapes for resolving and validating enum inputs across `Profile`, `IdentityScope`, `BackupComponent`, and `Tag`.

## Context

Consistency in naming shapes across boundaries (files/modules/types/functions) is a core first principle. Currently, `BackupComponent` provides `resolve_backup_component` and `validate_backup_component`. `Profile` has `resolve_profile`, `validate_hardware_profile` and `validate_profile`. `IdentityScope` has `resolve_identity_scope`, but validation happens implicitly or elsewhere. `Tag` uses `resolve_tags` but returns a `Vec<String>` instead of an `Option` or `Result`. The error messages returned on invalid inputs also drift in formatting.

## Evidence

- path: "src/domain/backup_component.rs"
  loc: "54-73"
  note: "Defines `resolve_backup_component` returning `Option<BackupComponent>` and `validate_backup_component` returning `Result<BackupComponent, AppError>`."

- path: "src/domain/profile.rs"
  loc: "44-67"
  note: "Defines `resolve_profile` returning `Option<Profile>` but then splits validation into `validate_hardware_profile` and `validate_profile`."

- path: "src/domain/identity.rs"
  loc: "61-65"
  note: "Defines `resolve_identity_scope` returning `Option<IdentityScope>`, but no dedicated `validate_identity_scope` exists."

- path: "src/domain/error.rs"
  loc: "21-36"
  note: "Error names and formatting drift slightly: `InvalidProfile(p) => invalid profile: {p}`, but `InvalidBackupComponent(t) => invalid backup component: {t}`."

## Change Scope

- `src/domain/backup_component.rs`
- `src/domain/profile.rs`
- `src/domain/identity.rs`
- `src/domain/error.rs`
