---
label: "refacts"
implementation_ready: false
---

## Goal
Relocate CLI-specific string input parsing and alias resolution out of core domain models and into the application CLI layer.

## Problem
Core domain models (`VcsIdentity`, `Profile`, `BackupTarget`) are currently responsible for parsing raw CLI strings and resolving command-line aliases. This violates Boundary Sovereignty by injecting UI/CLI concerns directly into the business logic layer.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "cli_aliases_domain_data_arch.md"
  path: "src/domain/vcs_identity.rs"
  loc: "SWITCH_IDENTITY_ALIASES"
  note: "Defines hardcoded CLI aliases within the domain model."
- source_event: "cli_aliases_domain_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "PROFILE_ALIASES"
  note: "Contains CLI aliases and mapping rules for profile resolution."
- source_event: "cli_aliases_domain_data_arch.md"
  path: "src/domain/backup_target.rs"
  loc: "from_input"
  note: "Implements string-based parsing and alias resolution directly on the domain type."

## Change Scope
- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/`

## Constraints
- Domain models must only represent valid domain states and rules.
- Input validation and alias resolution must be handled exclusively in the `app/cli/` adapter layer.

## Acceptance Criteria
- CLI aliases are moved out of domain models.
- Domain models instantiate only through fallible constructors or explicit conversion traits invoked post-parsing.
