---
label: "refacts"
implementation_ready: false
---

## Goal

Refactor the codebase to adhere to architecture naming conventions, avoiding ambiguous directory and code names (e.g., "core", "helpers"), and clarify documentation regarding role profiles.

## Problem

The documentation prohibits ambiguous names such as `core/`, `utils/`, and `helpers/`, but the `src/assets/ansible/roles/shell/config/common/alias/core/` directory and numerous source modules still use the term "helpers". Additionally, documentation incorrectly claims only the `brew` role uses profile-specific configs, contradicting the implementation of the `llm` role.

## Evidence

- source_event: "naming_rule_drift_consistency.md"
  path: "docs/architecture.md"
  loc: "line 67"
  note: "Documentation states 'Ambiguous names such as `core/`, `utils/`, `helpers/` are forbidden'."
- source_event: "naming_rule_drift_consistency.md"
  path: "src/assets/ansible/roles/shell/config/common/alias/core"
  loc: "Directory exists"
  note: "A directory named `core` exists in the codebase, violating the documented principle."
- source_event: "profile_config_drift_consistency.md"
  path: "docs/architecture.md"
  loc: "line 82"
  note: "Documentation explicitly claims `config/profiles/ (brew only)`."
- source_event: "profile_config_drift_consistency.md"
  path: "src/assets/ansible/roles/llm/config/profiles/"
  loc: "Directory exists"
  note: "The `llm` role contains profile-specific configurations, contradicting the documentation."
- source_event: "vague_names_taxonomy.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "line 270"
  note: "Uses the term `// Shared helpers` as a section divider for utility functions."
- source_event: "vague_names_taxonomy.md"
  path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "line 1"
  note: "Module documentation uses `//! Test helpers for mocking the environment.`"
- source_event: "vague_names_taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Subcommand documentation describes Git and GitHub commands as `/// Git helpers.` and `/// GitHub CLI helpers.`"
- source_event: "vague_names_taxonomy.md"
  path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Subcommand documentation describes Git and GitHub commands as `/// Git helpers.` and `/// GitHub CLI helpers.`"

## Change Scope

- `docs/architecture.md`
- `src/assets/ansible/roles/shell/config/common/alias/core/`
- `src/assets/ansible/roles/llm/config/profiles/`
- `src/app/commands/backup/mod.rs`
- `crates/mev-internal/src/testing/env_mock.rs`
- `src/app/cli/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`

## Constraints

- Ambiguous names like `core/`, `utils/`, or `helpers/` are strictly prohibited.
- Documentation must conform to implementation, or the implementation must be corrected to conform to the rules.

## Acceptance Criteria

- The `core` directory in Ansible configurations is renamed to something specific to its contents.
- The `docs/architecture.md` file correctly documents `config/profiles/` usage.
- Modules, comments, and documentation no longer use the term "helpers".