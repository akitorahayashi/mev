---
label: "refacts"
implementation_ready: false
---

## Goal

Refactor ambiguous taxonomy terms (`common`, `helpers`) into precise, domain-aligned language to comply with core design principles and improve comprehensibility.

## Problem

The terms `common` and `helpers` are used extensively across the domain, CLI definitions, and internal documentation. This violates the core design principle that prohibits ambiguous names such as `base`, `common`, `core`, `utils`, or `helpers`. Ambiguous taxonomies obscure a module or concept's precise responsibility and hinder refactoring safety. For example, `Profile::Common` should be `Profile::Baseline` or `Profile::Default`, and `Git helpers.` should be described more accurately as `Git integration commands.`.

## Evidence

- source_event: "remove_ambiguous_common_profile_taxonomy.md"
  path: "src/domain/profile.rs"
  loc: "line 12, 21, 56"
  note: "Defines the `Profile::Common` variant and its canonical mapping to `common`."
- source_event: "remove_ambiguous_common_profile_taxonomy.md"
  path: "src/app/cli/make.rs"
  loc: "line 14, 15"
  note: "Defines the profile argument with the default value `common`."
- source_event: "remove_ambiguous_helpers_cli_taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "line 67, 71"
  note: "Uses `Git helpers.` and `GitHub CLI helpers.` for the `Git` and `Gh` internal subcommands respectively."
- source_event: "remove_ambiguous_helpers_cli_taxonomy.md"
  path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "line 19, 23"
  note: "Uses `Git helpers.` and `GitHub CLI helpers.` for the `Git` and `Gh` subcommands respectively."
- source_event: "remove_ambiguous_helpers_taxonomy.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "line 275"
  note: "The comment reads `// Shared helpers` which explicitly uses the forbidden term `helpers`."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/make.rs`
- Config directory layouts under `src/assets/ansible/roles/**/config/common`
- `src/app/cli/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`

## Constraints

- Refactor `common` everywhere (structs, methods, text, folders) and rename correctly to a domain-aligned term.
- Remove `helpers` everywhere.

## Acceptance Criteria

- The term `common` is eradicated from `Profile` definitions, domain logic, command parameters, test cases, and configuration folder layouts.
- The term `helpers` is removed from all CLI descriptions and internal documentation comments.
