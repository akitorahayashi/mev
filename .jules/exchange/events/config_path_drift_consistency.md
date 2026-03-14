---
label: "docs"
created_at: "2026-03-14"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation for the `config create` command in `docs/usage.md` incorrectly states that it deploys all role configs to `~/.config/mev/`, whereas the CLI `--help` and underlying implementation (`src/adapters/identity_store/paths.rs`) show it deploys to `~/.config/mev/roles/`.

## Goal

Update `docs/usage.md` to accurately reflect the correct deployment path (`~/.config/mev/roles/`).

## Context

Consistency between the documentation and the implementation is critical. When users run `mev config create`, they expect the configs to be deposited exactly where the documentation indicates. The current documentation provides an inaccurate path, creating confusion about the config directory structure.

## Evidence

- path: "docs/usage.md"
  loc: "line 41"
  note: "The documentation states `mev config create         # Deploy all role configs to ~/.config/mev/`."
- path: "src/app/cli/config.rs"
  loc: "line 12"
  note: "The CLI documentation explicitly defines `Deploy role configs to ~/.config/mev/roles/.`."

## Change Scope

- `docs/usage.md`
