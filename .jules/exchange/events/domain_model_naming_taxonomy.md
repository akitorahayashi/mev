---
label: "refacts"
created_at: "2026-03-31"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The domain layer uses different conceptual naming conventions for CLI command targets, leading to overloaded and inconsistent terms like `Profile`, `SwitchIdentity`, `BackupTarget`, and `tag`.

## Goal

Establish a consistent naming convention and structural taxonomy for domain entities that act as targets or parameters for various system operations, ensuring a unified conceptual language.

## Context

The repository contains several domain types representing entities that can be targeted or operated on:
1. `Profile` (hardware profile like macbook, mac-mini).
2. `SwitchIdentity` (Git identity target like personal, work).
3. `BackupTarget` (backup operation target like system, vscode).
4. `tag` (Ansible task execution target).

These names lack consistency. Some use the command action (`SwitchIdentity`, `BackupTarget`), while others are standalone nouns (`Profile`, `tag`). The anti-pattern "Domain models must not be named after CLI command actions" is explicitly stated in the design rules: "(e.g., naming a type 'SwitchIdentity' because it is used by a 'switch' command is prohibited; use a domain noun instead)."

Renaming `SwitchIdentity` to something like `GitIdentityScope` (or `IdentityScope` / `IdentityProfile`), and `BackupTarget` to something like `BackupComponent` or `SystemComponent` would align better with the domain language.

## Evidence

- path: "src/domain/identity.rs"
  loc: "pub enum SwitchIdentity"
  note: "Named after the 'switch' CLI command, violating the domain language first rule."

- path: "src/domain/backup_target.rs"
  loc: "pub enum BackupTarget"
  note: "Contains 'Target' which is a generic overloaded term, and is closely tied to the 'backup' command action."

- path: "src/domain/profile.rs"
  loc: "pub enum Profile"
  note: "Standalone domain noun, correctly named."

## Change Scope

- `src/domain/identity.rs`
- `src/domain/backup_target.rs`
- `src/domain/error.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/cli/switch.rs`
- `src/app/cli/backup.rs`
- `src/app/api.rs`
