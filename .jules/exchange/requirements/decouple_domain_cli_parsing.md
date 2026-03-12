---
label: "refacts"
implementation_ready: false
---

## Goal

Move CLI-specific string input parsing logic and alias resolution out of core domain models and into the application or adapter CLI layers.

## Problem

Core domain models (`Profile`, `SwitchIdentity`, `BackupTarget`) currently contain logic to parse CLI input strings and aliases. This leaks UI/transport concerns into pure domain rules, violating boundary sovereignty.

## Evidence

- source_event: "domain_cli_parsing_data_arch.md"
  path: "src/domain/vcs_identity.rs"
  loc: "resolve_switch_identity"
  note: "`resolve_switch_identity` handle CLI input mapping in the domain."
- source_event: "domain_cli_parsing_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "resolve_profile, validate_machine_profile, validate_profile"
  note: "`resolve_profile` and `validate_machine_profile` embed CLI string mapping and validation logic."
- source_event: "domain_cli_parsing_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "from_input"
  note: "`from_input` parses user-provided strings, including an alias 'vscode-extensions'."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/`

## Constraints

- Domain types should only validate intrinsic business logic invariants, not map external CLI inputs.

## Acceptance Criteria

- CLI string resolution and alias mapping logic is removed from `src/domain/` components.
- CLI input parsing logic is handled at the boundary in `src/app/cli/`.
