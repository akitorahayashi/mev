---
label: "refacts"
---

## Goal

Consolidate the recursive directory copying logic by moving it into the `FsPort` abstraction and implementing it in `StdFs`.

## Problem

The function `copy_dir_recursive` is duplicated across multiple modules (`src/app/commands/config/mod.rs` and `src/app/commands/deploy_configs.rs`), bypassing the `FsPort` abstraction designed to decouple I/O concerns from domain and application layers. This violates both the Single Source of Truth principle (for I/O operations) and Boundary Sovereignty.

## Affected Areas

### Filesystem Port and Adapter

- `src/domain/ports/fs.rs`
- `src/adapters/fs/std_fs.rs`

### CLI Commands

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/create/mod.rs`

## Constraints

- Filesystem interactions from commands should run exclusively through the abstractions defined in `FsPort`.
- Functionality should be unified to prevent duplicate logic.

## Risks

- Regression in config deployment commands if the `FsPort` implementation behaves differently than the removed inline implementations. The previous `copy_dir_recursive` in `deploy_configs.rs` verified that the source was a directory and returned an `AppError::Config`, while `config/mod.rs` did not. The new logic should be centralized safely in `StdFs`.

## Acceptance Criteria

- `copy_dir_recursive` logic is entirely removed from `src/app/commands/config/mod.rs` and `src/app/commands/deploy_configs.rs`.
- `FsPort` is extended with a `copy_dir_recursive` method.
- `StdFs` implements the `copy_dir_recursive` logic, ensuring single ownership over this file I/O behavior.
- Calling sites in `config/mod.rs` and `deploy_configs.rs` use the `fs` from `DependencyContainer` or a passed-in `&dyn FsPort`.

## Implementation Plan

1. Modify `src/domain/ports/fs.rs` to include `fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<(), AppError>;` using `replace_with_git_merge_diff`.
2. Verify the modification in `src/domain/ports/fs.rs` by using `read_file`.
3. Modify `src/adapters/fs/std_fs.rs` to implement `copy_dir_recursive` mapping `std::fs` errors to `AppError::Io`, using `replace_with_git_merge_diff`.
4. Verify the modification in `src/adapters/fs/std_fs.rs` by using `read_file`.
5. Modify `src/app/commands/deploy_configs.rs` to add `fs: &dyn FsPort` to the `deploy_for_tags` signature, remove the inline `copy_dir_recursive`, and update the call site to use `fs.copy_dir_recursive(&source, &target)` using `replace_with_git_merge_diff`.
6. Verify the modification in `src/app/commands/deploy_configs.rs` by using `read_file`.
7. Modify `src/app/commands/make/mod.rs` to pass `&ctx.fs` when calling `deploy_configs::deploy_for_tags` using `replace_with_git_merge_diff`.
8. Verify the modification in `src/app/commands/make/mod.rs` by using `read_file`.
9. Modify `src/app/commands/create/mod.rs` to pass `&ctx.fs` when calling `deploy_configs::deploy_for_tags` using `replace_with_git_merge_diff`.
10. Verify the modification in `src/app/commands/create/mod.rs` by using `read_file`.
11. Modify `src/app/commands/config/mod.rs` to remove the standalone `copy_dir_recursive` and update the call site to use `ctx.fs.copy_dir_recursive` using `replace_with_git_merge_diff`.
12. Verify the modification in `src/app/commands/config/mod.rs` by using `read_file`.
13. Verify the changes compile and run correctly using `run_in_bash_session` to run `cargo test` and `just check`.
14. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
15. Remove the requirement file `.jules/exchange/requirements/consolidate_fs_port_copy_dir.md` using `run_in_bash_session` to delete the file.