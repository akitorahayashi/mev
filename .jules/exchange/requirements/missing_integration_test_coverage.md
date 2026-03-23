---
label: "tests"
implementation_ready: false
---

## Goal

Provide comprehensive integration test coverage for backup orchestrations, GitHub label management, and VCS identity switching to prevent silent failures and regressions.

## Problem

Critical parts of the application orchestration—including system/VSCode backups, GitHub label resetting/deployment, and VCS identity switching—have zero or near-zero test coverage. This lack of coverage creates a high risk of regressions and data loss when configurations shift.

## Evidence

- source_event: "untested_backup_operations_cov.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "59-247"
  note: "Only 9/149 lines covered. Critical methods like `execute_system`, `format_value`, and `execute_vscode` lack line-level tests, risking regressions when data formats change."
- source_event: "untested_gh_label_reset_cov.md"
  path: "crates/mev-internal/src/app/commands/gh/labels_reset.rs"
  loc: "16-32"
  note: "0/12 lines covered in a command that explicitly deletes all labels from a given GitHub repository."
- source_event: "untested_gh_label_reset_cov.md"
  path: "crates/mev-internal/src/app/commands/gh/labels_deploy.rs"
  loc: "17-38"
  note: "0/13 lines covered in a command that manipulates repository labels by replacing or creating them based on a bundled catalog."
- source_event: "untested_vcs_identity_switch_cov.md"
  path: "src/app/commands/switch/mod.rs"
  loc: "14-43"
  note: "0/23 lines covered. The `execute` function performs a critical auth/state transition but lacks integration tests validating the success or failure paths."

## Change Scope

- `src/app/commands/backup/mod.rs`
- `tests/cli_contracts/backup.rs` (to be created or modified)
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/tests/gh_contracts.rs` (to be created or modified)
- `src/app/commands/switch/mod.rs`
- `tests/cli_contracts/switch.rs` (to be created or modified)

## Constraints

- Tests must assert externally observable behavior at the owning boundary.
- Tests must not rely on mutating the developer's global system state.

## Acceptance Criteria

- Integration tests cover the success and failure paths for `backup`, `switch`, `gh labels deploy`, and `gh labels reset`.
- Test coverage ensures fallback logic, error handling, and file creation behaviors are validated.