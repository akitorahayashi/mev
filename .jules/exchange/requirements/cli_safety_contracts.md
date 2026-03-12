---
label: "feats"
implementation_ready: false
---

## Goal

Implement consistent safety contracts (`--dry-run` or explicit confirmation logic like `--yes`/`--force`) for destructive CLI operations.

## Problem

Commands that mutate system configuration (`create`, `make`, `switch`) lack standardized safety measures. They run without explicit warnings, confirmation prompts, or `--dry-run` capabilities, which increases the risk of users inadvertently modifying their system state.

## Evidence

- source_event: "destructive_ops_safety_contract_cli_sentinel.md"
  path: "src/app/cli/create.rs"
  loc: "pub struct CreateArgs { ... }"
  note: "Defines no `--dry-run` or `--yes` option, but executes an Ansible plan that changes the system."
- source_event: "destructive_ops_safety_contract_cli_sentinel.md"
  path: "src/app/cli/make.rs"
  loc: "pub struct MakeArgs { ... }"
  note: "Lacks `--dry-run` or safety confirmation before executing targeted Ansible changes."
- source_event: "destructive_ops_safety_contract_cli_sentinel.md"
  path: "src/app/cli/switch.rs"
  loc: "pub struct SwitchArgs { ... }"
  note: "Silently switches identity profiles and modifies the Git system config without asking for user consent."

## Change Scope

- `src/app/cli/create.rs`
- `src/app/cli/make.rs`
- `src/app/cli/switch.rs`
- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/switch/mod.rs`

## Constraints

- Implement a mechanism to prompt users before taking destructive actions unless a bypass flag is provided.

## Acceptance Criteria

- Destructive operations require confirmation or an explicit override flag (e.g., `--yes`).
- Where applicable, a `--dry-run` flag is implemented to output what actions would be taken.
