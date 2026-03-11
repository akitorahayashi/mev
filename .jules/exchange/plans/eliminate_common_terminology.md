---
label: "refacts"
---

## Goal

Rename the ambiguous term "common" to "default" across the codebase to clearly describe its exact responsibility as a default or fallback configuration.

## Problem

The term "common" is used pervasively throughout the codebase as a generic profile fallback (`Profile::Common`), a subpath for backup targets, and a subdirectory for Ansible role configurations. This violates the design rule against ambiguous names that restate package/directory scope or hide true capabilities.

## Affected Areas

### Rust Domain & App
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/make.rs`

### Ansible Configurations
- `src/assets/ansible/roles/*/config/common`
- `src/assets/ansible/roles/*/tasks/*.yml` (and other playbook files referencing `common`)
- `src/assets/ansible/roles/*/AGENTS.md` (and other docs referencing `common`)

## Constraints

- Files and classes must identify single, specific responsibilities.
- The terms `base`, `common`, `core`, `utils`, and `helpers` are strictly avoided.
- A final comprehensive search is required after renaming/deleting structures to ensure no dead references.

## Risks

- Missed references to the `common` path in Ansible tasks could cause provisioning failures.
- CLI usage of `--profile common` would break if not properly aliased or updated to `--profile default`.

## Acceptance Criteria

- `Profile::Common` is renamed to `Profile::Default` or similar intention-revealing name.
- Backup targets no longer use the literal `common` in their paths or structures (e.g., they use `default`).
- Ansible roles configuration directories are restructured so `common` is renamed to `default`.
- All Ansible tasks and playbooks referencing `/common/` are updated.

## Implementation Plan

1. Use `run_in_bash_session` with `find src/assets/ansible/roles -type d -name "common"` and `mv` to rename all `config/common` directories to `config/default`.
2. Use `run_in_bash_session` with `rg -l 'common' src/assets/ansible/roles | xargs sed -i '' 's/common/default/g'` (or similar Linux `sed`) to update all file paths and string references in Ansible roles from `common` to `default`.
3. Use `replace_with_git_merge_diff` to update `src/domain/profile.rs`. Rename `Profile::Common` to `Profile::Default`. Update its canonical string to `"default"`. Update aliases to `def`. Update all tests and references in the file.
4. Use `replace_with_git_merge_diff` to update `src/domain/backup_target.rs`. Change the string returned by `subpath()` from `"common"` to `"default"`.
5. Use `replace_with_git_merge_diff` to update `src/app/cli/make.rs`. Change the default value of the profile argument from `"common"` to `"default"`, and update the help text to `(default, macbook/mbk, mac-mini/mmn)`.
6. Run `cargo test` and `cargo check` to ensure no compile errors in Rust code.
7. Use `run_in_bash_session` to run a comprehensive search (`grep -ri common src/`) to verify no ambiguous uses of "common" remain in the codebase.
