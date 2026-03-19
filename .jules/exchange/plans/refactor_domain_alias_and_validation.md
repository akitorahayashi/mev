---
label: "refacts"
---

## Goal

Consolidate alias resolution logic and explicit boundary validation within domain models (`BackupTarget`, `Profile`, `SwitchIdentity`) to prevent redundant implementations, eliminate domain rules leaking into CLI logic, and ease schema evolution.

## Current State

Duplicate and inconsistent alias resolution exists across domain models, and validation logic leaks into CLI orchestration.
- `src/domain/vcs_identity.rs`: Implements custom alias resolution loop for `SwitchIdentity` using an array of tuples.
- `src/domain/profile.rs`: Implements custom alias resolution loop for `Profile` using an array of tuples. It also correctly implements a validation boundary (`Result<Profile, AppError>`).
- `src/domain/backup_target.rs`: Implements custom alias resolution using a match statement on strings. It lacks an explicit validation boundary (returns `Option`), forcing the caller to handle the failure and construct the error. It also has an `all()` function returning a hardcoded array, complicating schema evolution.
- `src/domain/error.rs`: Lacks an `InvalidBackupTarget` error variant.
- `src/app/commands/backup/mod.rs`: The CLI orchestration manually unwraps the `Option<BackupTarget>` and builds an error message, leaking domain rules.

## Plan

1. **Update `AppError`**
   - In `src/domain/error.rs`, add an `InvalidBackupTarget(String)` variant to the `AppError` enum.
   - Update `fmt::Display` implementation for `AppError::InvalidBackupTarget`.
2. **Refactor `BackupTarget` domain model**
   - In `src/domain/backup_target.rs`, introduce a constant array `BACKUP_TARGET_ALIASES` mapping strings to `BackupTarget` variants, similar to `PROFILE_ALIASES`.
   - Update `BackupTarget::from_input` (or replace with `resolve_backup_target`) to iterate over `BACKUP_TARGET_ALIASES` for resolution.
   - Add a `validate_backup_target` function that calls the resolver and returns `Result<BackupTarget, AppError::InvalidBackupTarget>`.
   - Consider using `strum` macros (`EnumIter`, etc.) if available to auto-generate `all()` and parse strings, OR refactor `all()` to use the enum variants directly to reduce maintenance burden. Since strum might not be a dependency, we will consolidate the match arms and arrays. Let's explicitly define a `try_from` or `parse` with explicit validation.
   - For `BackupTarget`: create a static array `ALL_TARGETS` and use it for `all()`.
3. **Refactor Alias Resolution Strategy**
   - Rather than creating a generic macro/trait that might be overkill, we will ensure that `BackupTarget` matches the explicit boundary pattern of `Profile`.
   - Update `BackupTarget::from_input` to `resolve_backup_target` (to match `resolve_profile` and `resolve_switch_identity` naming convention).
   - Add `validate_backup_target` in `src/domain/backup_target.rs` that returns `Result<BackupTarget, AppError>`.
4. **Update CLI Orchestration**
   - In `src/app/commands/backup/mod.rs`, replace the manual `Option` unwrapping and error construction.
   - Call `validate_backup_target(target_input)` directly and use the resulting `BackupTarget`. Let `AppError` propagate up.
5. **Update Tests**
   - Update tests in `src/domain/backup_target.rs` to cover `resolve_backup_target` and `validate_backup_target`.
   - Ensure tests verify explicit boundary failure.

## Acceptance Criteria

- Alias resolution is centralized and uniformly utilized across `Profile`, `SwitchIdentity`, and `BackupTarget`.
- `BackupTarget` implements explicit boundaries returning `AppError::InvalidBackupTarget`.
- CLI (`src/app/commands/backup/mod.rs`) does not unwrap `Option` for `BackupTarget` and does not construct the error message manually.
- New `BackupTarget` entries only require updating minimal locations.

## Risks

- Breaking existing CLI commands if alias mappings are accidentally changed. (Mitigated by test coverage).
- Breaking existing backup workflows if error propagation is flawed. (Mitigated by explicit `AppError` mapping).
