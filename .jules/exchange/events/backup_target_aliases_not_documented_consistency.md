---
label: "docs"
created_at: "2026-03-11"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation for the `backup` command in `docs/usage.md` does not mention the valid `vscode-extensions` backup target alias.

## Goal

Document the valid target alias `vscode-extensions` in the `backup` section of `docs/usage.md`.

## Context

The implementation of `BackupTarget::from_input` in `src/domain/backup_target.rs` explicitly allows `"vscode"` and `"vscode-extensions"` as valid inputs for the VSCode backup target. However, `docs/usage.md` only lists `mev backup vscode`.

## Evidence

- path: "src/domain/backup_target.rs"
  loc: "line 15"
  note: "Defines `\"vscode\" | \"vscode-extensions\" => Some(Self::Vscode)`"
- path: "docs/usage.md"
  loc: "lines 45-48"
  note: "Only lists `mev backup vscode` and omits the alias."

## Change Scope

- `docs/usage.md`
