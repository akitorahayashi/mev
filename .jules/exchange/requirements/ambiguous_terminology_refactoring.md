---
label: "refacts"
implementation_ready: false
---

## Goal

Replace occurrences of vague terminology like "base", "core", and "helpers" across the codebase and documentation with precise, domain-specific terms.

## Problem

The terms "base", "core", and "helpers" are used ambiguously across the codebase (e.g., paths/URLs, documentation headers, and delegating CLI commands). This violates taxonomy and naming principles, leading to confusion about component responsibilities.

## Evidence

- source_event: "ambiguous_base_term_usage_taxonomy.md"
  path: "src/adapters/identity_store/paths.rs"
  loc: "line 3"
  note: "Uses 'base path' instead of a more specific term like 'configuration root'."
- source_event: "ambiguous_base_term_usage_taxonomy.md"
  path: "src/assets/ansible/roles/rust/config/common/tools.yml"
  loc: "line 6"
  note: "Uses 'release base URL' instead of 'release root URL'."
- source_event: "ambiguous_core_term_usage_taxonomy.md"
  path: "src/assets/ansible/roles/nodejs/config/common/coder/skills/svo-cli-design/SKILL.md"
  loc: "line 8, 10"
  note: "Uses 'Core Objective' and 'core required inputs'. These can be replaced with 'Primary Objective' and 'fundamental required inputs'."
- source_event: "ambiguous_core_term_usage_taxonomy.md"
  path: "src/assets/ansible/roles/nodejs/config/common/coder/skills/effective-prompting/SKILL.md"
  loc: "line 8"
  note: "Uses 'Core Objective' as a section header."
- source_event: "ambiguous_helper_term_usage_taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Uses 'helpers' to describe subcommands delegating to `git` and `gh` CLIs. A more precise term like 'commands' or 'integrations' would be better."
- source_event: "ambiguous_helper_term_usage_taxonomy.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "line 274"
  note: "Uses '// Shared helpers' as a section comment for a function resolving paths."
- source_event: "ambiguous_helper_term_usage_taxonomy.md"
  path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Repeats the same use of 'helpers' for git and gh CLI commands as the main crate."

## Change Scope

- `src/adapters/identity_store/paths.rs`
- `src/assets/ansible/roles/rust/config/common/tools.yml`
- `src/assets/ansible/roles/nodejs/config/common/coder/skills/svo-cli-design/SKILL.md`
- `src/assets/ansible/roles/nodejs/config/common/coder/skills/effective-prompting/SKILL.md`
- `src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`

## Constraints

- Code changes must not break existing integrations or data contracts.
- Documentation structure updates should remain declarative.

## Acceptance Criteria

- All identified occurrences of "base", "core", and "helpers" are replaced with specific, domain-appropriate terminology.
- The project's tests pass with the new nomenclature.
