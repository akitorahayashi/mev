---
label: "refacts"
---

## Goal

Decouple application command orchestration from concrete filesystem and Ansible adapter internals by using port abstractions (`FsPort`, `AnsiblePort`), enabling unit testing with in-process test doubles.

## Current State

Application commands directly use `std::fs` and construct Ansible role paths internally, bypassing port boundaries. They also depend on a concrete `DependencyContainer` which exposes internal paths publicly (`pub ansible_dir`), making orchestration logic untestable in isolation without slow, brittle integration tests.
- `src/app/commands/deploy_configs.rs`: Uses `std::fs::remove_dir_all`, `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy` instead of port abstractions. It directly receives `ansible_dir: &Path` instead of taking `fs: &dyn FsPort` to copy files.
- `src/app/commands/config/mod.rs`: `copy_dir_recursive` and `deploy` use `std::fs` calls instead of `FsPort`. Directly constructs paths using `ctx.ansible_dir.join(...)`.
- `src/app/container.rs`: The `DependencyContainer` struct leaves `pub ansible_dir: PathBuf` exposed, allowing commands to bypass `AnsiblePort` for asset paths.
- `src/domain/ports/fs.rs`: The `FsPort` lacks `remove_dir_all`, `copy`, and `rename` operations required by the commands.
- `src/domain/ports/ansible.rs`: The `AnsiblePort` lacks a method to resolve the config directory of a role without exposing the raw root `ansible_dir`.
- `src/app/commands/backup/mod.rs`: Uses `ctx.ansible_dir` directly to locate package default definitions.
- `src/app/commands/create/mod.rs`: Tightly couples logic by passing `&ctx.ansible_dir` to `deploy_configs`.
- `src/app/commands/make/mod.rs`: Tightly couples logic by passing `&ctx.ansible_dir` to `deploy_configs`.
- `tests/cli/backup.rs`: Contract tests expect failure due to missing ansible assets when they should ideally be insulated from the actual filesystem or the failure is a side effect of poor isolation.
- `src/testing/`: Lacks fast test doubles for orchestration unit tests for `config` and `deploy_configs`.

## Plan

1. Update `src/domain/ports/fs.rs`:
   - Add `remove_dir_all(&self, path: &Path) -> Result<(), AppError>;` to `FsPort`.
   - Add `copy(&self, from: &Path, to: &Path) -> Result<u64, AppError>;` to `FsPort`.
   - Add `rename(&self, from: &Path, to: &Path) -> Result<(), AppError>;` to `FsPort`.
2. Update `src/adapters/fs/std_fs.rs`:
   - Implement the new `remove_dir_all`, `copy`, and `rename` methods in `StdFs` using `std::fs` equivalents.
3. Update `src/domain/ports/ansible.rs`:
   - Add `role_config_dir(&self, role: &str) -> Option<PathBuf>;` to `AnsiblePort`.
4. Update `src/adapters/ansible/executor.rs`:
   - Implement `role_config_dir` in `AnsibleAdapter` to return `self.roles_dir.join(role).join("config")` if the role exists (or just constructs the path).
5. Update `src/app/container.rs`:
   - Change `pub ansible_dir: PathBuf` to private `ansible_dir: PathBuf` in `DependencyContainer`.
6. Update `src/app/commands/deploy_configs.rs`:
   - Change `deploy_for_tags` signature: remove `ansible_dir: &Path` and add `fs: &dyn FsPort`.
   - Refactor `deploy_for_tags` to use `ansible.role_config_dir(&role)` instead of `ansible_dir.join(...)`.
   - Refactor `copy_dir_recursive` to take `fs: &dyn FsPort` and use its methods instead of `std::fs`.
   - Replace all `std::fs` calls with `fs` method calls in `deploy_for_tags`.
7. Update `src/app/commands/config/mod.rs`:
   - Refactor `deploy` to use `ctx.fs` and `ctx.ansible.role_config_dir(&role)` exclusively.
   - Refactor or remove its internal `copy_dir_recursive` to use `ctx.fs` instead of `std::fs`.
8. Update `src/app/commands/backup/mod.rs`:
   - In `resolve_definitions_dir`, replace `ctx.ansible_dir.join(...)` with `ctx.ansible.role_config_dir(target.role()).map(|p| p.join(target.subpath()).join("definitions"))`. Handle `Option` appropriately (e.g., fallback or error if role config dir not found).
9. Update `src/app/commands/create/mod.rs` and `src/app/commands/make/mod.rs`:
   - Update `deploy_configs::deploy_for_tags` calls to pass `&ctx.fs` instead of `&ctx.ansible_dir`.
10. Update test doubles in `src/testing/`:
    - Ensure `MockFsPort` (if it exists, or create one if needed) implements the new `FsPort` methods.
    - Ensure `MockAnsiblePort` implements the new `role_config_dir`.
11. Add unit tests for `deploy_configs` and `config::deploy` in `src/app/commands/deploy_configs.rs` and `src/app/commands/config/mod.rs` using test doubles, verifying behavior without hitting the real filesystem.
12. Verify that `tests/cli/backup.rs` test `backup_alias_bk_is_accepted` still passes or update it if behavior changes to explicitly rely on the new abstractions correctly.

## Acceptance Criteria

- `deploy_configs` and `config` commands use `FsPort` and `AnsiblePort` exclusively.
- `DependencyContainer` encapsulates `ansible_dir` (it is no longer `pub`).
- Application commands have unit tests using fast test doubles in `src/testing/`.
- No `std::fs` calls exist directly inside `src/app/commands/`.
- `tests/cli/backup.rs` alias test passes and is insulated appropriately.

## Risks

- The `role_config_dir` method on `AnsiblePort` returns an `Option<PathBuf>`, which may require additional error handling in `backup/mod.rs` if a role does not exist.
- Replacing `std::fs` with `FsPort` in `copy_dir_recursive` may introduce slight differences in error handling (e.g., `AppError::Io` vs direct `io::Error`), so care must be taken to maintain the same error behavior.
- In-process test doubles for `FsPort` may not perfectly replicate all filesystem edge cases, requiring careful mock design.
