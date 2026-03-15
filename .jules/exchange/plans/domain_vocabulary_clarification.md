---
label: "refacts"
---

## Goal

Replace ambiguous "common" and "helpers" naming conventions across the project to explicitly reflect domain boundaries and responsibilities.

## Current State

- `src/domain/backup_target.rs`: Hardcodes `common` as the subdirectory within the role config directory. It should be changed to reflect `global`.
- `src/domain/profile.rs`: Defines a `Common` profile alongside machine profiles (`Macbook`, `MacMini`). It should be renamed to `Workspace` (alias: `wsc`). Tests check for `common` and `cmn`.
- `src/assets/ansible/roles/*/config/common`: Numerous configuration directories use `common` to store global or shared settings. These should be renamed to `global`.
- `src/app/commands/list/mod.rs`: Handles the `Profile::Common` display logic with a specific check.
- `src/app/commands/make/mod.rs`: Checks against `Profile::Common` to determine if a profile should be printed.
- `src/app/commands/backup/mod.rs`: Contains a generic `// Shared helpers` comment (line 274).
- `src/app/cli/mod.rs`: Contains generic `/// Git helpers.` (line 67) and `/// GitHub CLI helpers.` (line 71) comments.

## Plan

1. Rename `Profile::Common` to `Profile::WorkspaceConfig` in `src/domain/profile.rs`, updating string aliases to `"workspace"` and `"wsc"`. Update the test cases to match.
2. Update `src/app/commands/list/mod.rs` and `src/app/commands/make/mod.rs` to reference `Profile::WorkspaceConfig` instead of `Profile::Common`.
3. Rename all `src/assets/ansible/roles/*/config/common` directories to `src/assets/ansible/roles/*/config/global` using `git mv`.
4. Update `src/domain/backup_target.rs` `subpath()` to return `"global"`.
5. Run a codebase-wide find and replace (using `sed` or `rg` with replacements) in `src/assets/ansible/roles/` to change paths referencing `/common/` to `/global/` and specific file names or variables containing `common` (e.g. `llm_common_models`) to `global`.
6. Update `src/app/commands/backup/mod.rs` to replace `// Shared helpers` with `// Resolution and Listing Operations`.
7. Update `src/app/cli/mod.rs` to replace `/// Git helpers.` with `/// Internal Git operations.` and `/// GitHub CLI helpers.` with `/// Internal GitHub operations.`.
8. Verify changes by executing the test suite with `just test`.
9. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- `Profile::Common` is replaced with `Profile::WorkspaceConfig`.
- All `config/common` directories inside `src/assets/ansible/roles` are renamed to `config/global`.
- File paths inside Ansible tasks and variables referencing `common` are updated to `global`.
- Ambiguous comments are updated to describe domain actions.

## Risks

- Missed hardcoded `common` strings in paths within the Ansible tasks leading to failed playbooks.
