---
label: "refacts"
implementation_ready: false
---

## Goal

Generate tag groups, backup targets, and profiles dynamically from authoritative sources (like the Ansible playbook) instead of hardcoding them in the Rust domain layer.

## Problem

Ansible execution tags, backup targets, and profiles are hardcoded in the Rust domain layer (`src/domain/tag.rs`, `src/domain/backup_target.rs`, `src/domain/profile.rs`). This violates the single source of truth principle and the design rule against hardcoded enumerable values, leading to a maintenance burden when configurations change.

## Evidence

- source_event: "duplicate_tag_definitions_data_arch.md"
  path: "src/domain/tag.rs"
  loc: "tag_groups"
  note: "Defines hardcoded `tag_groups()` that duplicate information natively present in or derived from the playbook."
- source_event: "duplicate_tag_definitions_data_arch.md"
  path: "src/domain/tag.rs"
  loc: "FULL_SETUP_TAGS"
  note: "Defines hardcoded `FULL_SETUP_TAGS` that duplicate information natively present in or derived from the playbook."
- source_event: "duplicate_tag_definitions_data_arch.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter::new"
  note: "The `AnsibleAdapter::new` function uses logic to parse `playbook.yml` to extract roles and tags, showing that the playbook is already read and parsed at runtime."
- source_event: "hardcoded_enumerables_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "Profile"
  note: "Hardcoded enum `Profile` with `Macbook`, `MacMini`, `Common` variants, and hardcoded aliases."
- source_event: "hardcoded_enumerables_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "BackupTarget"
  note: "Hardcoded enum `BackupTarget` with `System`, `Vscode` variants."

## Change Scope

- `src/domain/tag.rs`
- `src/adapters/ansible/executor.rs`
- `src/app/commands/create/mod.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`

## Constraints

- Never hardcode enumerable values; generate them dynamically from the Ansible playbook.
- Systemic fixes are preferred over patches.
- The `AnsibleAdapter` logic can be extended to derive these values.

## Acceptance Criteria

- `src/domain/tag.rs`, `src/domain/profile.rs`, and `src/domain/backup_target.rs` do not contain hardcoded variants for configurations present in Ansible assets.
- Tags, profiles, and backup targets are successfully loaded dynamically at runtime.