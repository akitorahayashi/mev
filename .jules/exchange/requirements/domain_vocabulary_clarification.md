---
label: "refacts"
implementation_ready: false
---

## Goal
Replace ambiguous "common" and "helpers" naming conventions across the project to explicitly reflect domain boundaries and responsibilities.

## Problem
The repository relies on generic terminology that violates the project's naming conventions. The term `common` is used for generic configuration directories and conflated with machine profiles (`Profile::Common`). Additionally, `helpers` is used loosely without describing the specific domain responsibility, confusing the actual intent of modules and configs.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "ambiguous_common_paths_taxonomy.md"
  path: "src/domain/backup_target.rs"
  loc: "pub fn subpath"
  note: "Hardcodes `common` as the subdirectory within the role config directory."
- source_event: "ambiguous_common_paths_taxonomy.md"
  path: "src/assets/ansible/roles/system/config/common/system.yml"
  loc: "com.apple.WindowManager"
  note: "Example of a `common` directory used to store system definitions."
- source_event: "ambiguous_profile_common_taxonomy.md"
  path: "src/domain/profile.rs"
  loc: "Profile::Common"
  note: "`Common` is defined as a `Profile` variant alongside `Macbook` and `MacMini`."
- source_event: "generic_helpers_nomenclature_taxonomy.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "// Shared helpers"
  note: "Contains the generic comment `// Shared helpers` instead of describing the domain logic."
- source_event: "generic_helpers_nomenclature_taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "/// Git helpers."
  note: "Describes internal git commands generically as `/// Git helpers.`"

## Change Scope
- `src/domain/backup_target.rs`
- `src/domain/profile.rs`
- `src/assets/ansible/roles/*/config/*`
- `src/app/commands/list/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/cli/mod.rs`

## Constraints
- Terms like "common", "utils", "base", or "helpers" must not be used for naming models or directories.

## Acceptance Criteria
- `Profile::Common` is replaced with a distinct foundational concept (e.g., `WorkspaceConfig`).
- `common` role directories are renamed to specify their intent (e.g., `global`, `shared`).
- Module documentation accurately describes domain noun actions rather than using "helpers".
