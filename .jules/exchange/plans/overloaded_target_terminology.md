---
label: "refacts"
---

## Goal

Standardize terminology by replacing the generic term "target" with more specific domain nouns where appropriate to prevent conceptual overloading.

## Current State

The word `target` is heavily overloaded across the codebase, representing a GitHub repository, a backup component, a destination path, and a numeric parsed value in different modules.
- `crates/mev-internal/src/domain/repo_target.rs`: Uses 'target' in the module name to describe a repository resolution process.
- `src/app/cli/backup.rs`: Uses `target` as the CLI struct field for a BackupComponent.
- `src/app/commands/backup/system.rs`: Uses `target` as a local variable when parsing numeric values.
- `src/app/commands/deploy_configs.rs`: Uses `target` as a local variable for destination directory path.

## Plan

1. Rename `crates/mev-internal/src/domain/repo_target.rs` to `crates/mev-internal/src/domain/repo_resolution.rs`.
2. Update `crates/mev-internal/src/domain/mod.rs` to use `pub mod repo_resolution` instead of `pub mod repo_target`.
3. Update `crates/mev-internal/src/app/commands/gh/labels_deploy.rs` to import and use `repo_resolution::resolve_repo_ref`.
4. Update `crates/mev-internal/src/app/commands/gh/labels_reset.rs` to import and use `repo_resolution::resolve_repo_ref`.
5. Update `src/app/cli/backup.rs` to rename the `target` field to `component` in the `BackupArgs` struct. Retain backwards-compatibility in CLI by configuring clap `#[arg(name = "target")]` on the struct field, while changing the Rust struct field name to `component`.
6. Update `src/app/commands/backup/system.rs` inside `format_numeric` to rename the local variable `target` to `value_str`.
7. Update `src/app/commands/deploy_configs.rs` to rename local variable `target` to `dest_dir`.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The generic word 'target' is replaced with 'component' for backups, 'repo_resolution' for repository logic, 'value_str' in the formatting function, and 'dest_dir' in deploy configs.
- The original requirement file is deleted.
