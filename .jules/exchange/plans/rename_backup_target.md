---
label: "refacts"
---

## Goal

Rename `backup_target` to `backup_scope` to align it with domain terminology and distinguish it from build/repo targets.

## Current State

- `src/domain/backup_target.rs`: Defines the `BackupTarget` enum to represent backup entities like `System` and `Vscode`. Overloads the word "target".
- `src/domain/error.rs`: Uses `InvalidBackupTarget` variant for `AppError`, which overloads "target".
- `src/domain/mod.rs`: Exports `backup_target` module.
- `src/app/commands/backup/mod.rs`: Uses `BackupTarget` and `validate_backup_target`, contributing to the overloaded term.
- `src/app/cli/backup.rs`: Has `target` CLI argument, and uses `target` variable name.
- `src/app/api.rs`: `backup` and `backup_list` use the term "target".
- `tests/cli/backup.rs`: Tests output containing the word "target".
- `tests/AGENTS.md`: Mentions `backup_target.rs`.

## Plan

1. Rename the file `src/domain/backup_target.rs` to `src/domain/backup_scope.rs`.
2. Verify the rename operation with `ls -l src/domain/backup_scope.rs`.
3. Update `src/domain/backup_scope.rs` to replace `BackupTarget` with `BackupScope`, `target` with `scope`, `TARGETS` with `SCOPES`. Rename `ALL_TARGETS` to `ALL_SCOPES`, `BACKUP_TARGET_ALIASES` to `BACKUP_SCOPE_ALIASES`, `resolve_backup_target` to `resolve_backup_scope`, `validate_backup_target` to `validate_backup_scope`. Rename all test methods (e.g., `backup_target_resolves_system` to `backup_scope_resolves_system`, etc.).
4. Update `src/domain/mod.rs` to replace `pub mod backup_target;` with `pub mod backup_scope;`.
5. Update `src/domain/error.rs` to rename `InvalidBackupTarget` to `InvalidBackupScope`. Also replace "invalid backup target" with "invalid backup scope" in `fmt::Display`.
6. Update `src/app/commands/backup/mod.rs` to use `BackupScope` and import from `crate::domain::backup_scope`. Rename variables and log messages from `target` to `scope`. Rename `list_targets` to `list_scopes`.
7. Update `src/app/cli/backup.rs` to rename the `target` argument to `scope`. Update `arg` macros, log messages and variable names. Update `--list` help message to say "List available backup scopes". Update `ArgGroup` "target" to "scope". Keep aliases or flags the same unless they conflict.
8. Update `src/app/api.rs` to rename the argument of `backup` from `target` to `scope`. Rename `backup_list` doc comment to mention "backup scopes".
9. Update `tests/cli/backup.rs` to use "scope" instead of "target". Update the test `backup_help_shows_target_argument` to `backup_help_shows_scope_argument` and check for "scope". Update `backup_list_shows_targets` to `backup_list_shows_scopes` and check for "Available backup scopes" instead of "Available backup targets". Update `backup_short_list_flag_shows_targets` to `backup_short_list_flag_shows_scopes` and check for "Available backup scopes". Update `backup_unknown_target_fails` to `backup_unknown_scope_fails` and check for "is not a valid scope" instead of "is not a valid target".
10. Update `tests/AGENTS.md` to reference `backup_scope.rs` instead of `backup_target.rs`.
11. Run `cargo test` and `cd crates/mev-internal && cargo test` to verify the refactoring was successful and no regressions were introduced.
12. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- `BackupTarget` is renamed to `BackupScope`.
- `backup_target.rs` is renamed to `backup_scope.rs`.
- `AppError::InvalidBackupTarget` is renamed to `AppError::InvalidBackupScope`.
- The CLI outputs and tests reflect the change from "target" to "scope".
- All tests pass.