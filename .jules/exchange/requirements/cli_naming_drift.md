---
label: "refacts"
implementation_ready: false
---

## Goal
Rename the `mev config create` subcommand to accurately reflect its deployment action.

## Problem
The `mev config create` command erroneously uses the "create" verb for deployment actions, conflicting with the top-level `mev create` command.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "config_create_naming_drift_cli_sentinel.md"
  path: "src/app/cli/config.rs"
  loc: "ConfigCommand::Create"
  note: "The enum variant and CLI command are named 'Create', but the docstring states 'Deploy role configs'."

## Change Scope
- `src/app/cli/config.rs`

## Constraints
- Command naming must not use synonyms for distinct actions.

## Acceptance Criteria
- `mev config create` is renamed to accurately describe its action (e.g., `deploy`).
