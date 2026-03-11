---
label: "refacts"
implementation_ready: false
---

## Goal

Rename ambiguous terms like "common" and "helpers" across the codebase to specific, domain-aligned nouns or verbs that clearly describe their exact responsibility.

## Problem

The terms "common" and "helpers" are used pervasively throughout the codebase as directory names, file names, profile identifiers, CLI subcommands, and internal functions. This violates the design rule against ambiguous names or responsibilities that restate package/directory scope or hide true capabilities.

## Evidence

- source_event: "common_naming_violation_taxonomy.md"
  path: "src/domain/profile.rs"
  loc: "12"
  note: "Defines `Profile::Common`, which is used as a generic profile fallback rather than a specific machine configuration."
- source_event: "common_naming_violation_taxonomy.md"
  path: "src/domain/backup_target.rs"
  loc: "53"
  note: "Uses `common` as a hardcoded subpath for backup targets, hiding the actual responsibility."
- source_event: "common_naming_violation_taxonomy.md"
  path: "src/assets/ansible/roles/*/config/common"
  loc: "directory structure"
  note: "Almost all Ansible roles have a `common` subdirectory for shared configurations."
- source_event: "helpers_naming_violation_taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "67, 71"
  note: "Uses 'helpers' as the help string for the `Git` and `Gh` subcommands."
- source_event: "helpers_naming_violation_taxonomy.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "274"
  note: "Uses '// Shared helpers' as a comment block to group functions, avoiding the assignment of those functions to specific responsibilities."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/assets/ansible/roles/*/config/common`
- `src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`

## Constraints

- Files and classes must identify single, specific responsibilities.
- The terms `base`, `common`, `core`, `utils`, and `helpers` are strictly avoided.
- A final comprehensive search is required after renaming/deleting structures to ensure no dead references.

## Acceptance Criteria

- `Profile::Common` is replaced with a specific, intention-revealing name or refactored away.
- Backup targets no longer use the literal `common` in their paths or structures.
- Ansible roles configuration directories are restructured so `common` is not used.
- Help strings for `Git` and `Gh` subcommands describe exactly what they do, omitting the word "helpers".
- Shared functions in `src/app/commands/backup/mod.rs` are explicitly named for their tasks or moved to dedicated modules, removing "helpers" terminology.
