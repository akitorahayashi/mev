---
label: "docs"
implementation_ready: true
---

## Goal

Update `docs/usage.md` to accurately reflect the correct deployment path (`~/.config/mev/roles/`).

## Problem

The documentation for the `config create` command in `docs/usage.md` incorrectly states that it deploys all role configs to `~/.config/mev/`, whereas the CLI `--help` and underlying implementation (`src/adapters/identity_store/paths.rs`) show it deploys to `~/.config/mev/roles/`.

## Context

Consistency between the documentation and the implementation is critical. When users run `mev config create`, they expect the configs to be deposited exactly where the documentation indicates. The current documentation provides an inaccurate path, creating confusion about the config directory structure.

## Evidence

- source_event: "config_path_drift_consistency.md"
  path: "docs/usage.md"
  loc: "line 41"
  note: "The documentation states `mev config create         # Deploy all role configs to ~/.config/mev/`."
- source_event: "config_path_drift_consistency.md"
  path: "src/app/cli/config.rs"
  loc: "line 12"
  note: "The CLI documentation explicitly defines `Deploy role configs to ~/.config/mev/roles/.`."

## Change Scope

- `docs/usage.md`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
