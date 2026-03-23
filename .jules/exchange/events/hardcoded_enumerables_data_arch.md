---
label: "refacts"
created_at: "2024-03-23"
author_role: "data_arch"
confidence: "high"
---

## Problem

Backup targets (`system`, `vscode`) and profiles (`macbook`, `mac-mini`, `common`) are hardcoded in the Rust domain layer, requiring manual updates to the Rust code whenever a new backup role or configuration profile is added to the Ansible assets.

## Goal

Eliminate hardcoded enumerable values for Profiles and Backup Targets by generating them dynamically from the Ansible assets.

## Context

`src/domain/profile.rs` and `src/domain/backup_target.rs` contain hardcoded enum variants (`Profile::Macbook`, `BackupTarget::System`, etc.). This violates the Single Source of Truth principle and the design rule against hardcoded enumerable values. The set of available profiles should be derived from the available configurations (e.g., from the authoritative Ansible assets), and the set of backup targets should be derived from the Ansible roles that support backups.

## Evidence

- path: "src/domain/profile.rs"
  loc: "Profile"
  note: "Hardcoded enum `Profile` with `Macbook`, `MacMini`, `Common` variants, and hardcoded aliases."
- path: "src/domain/backup_target.rs"
  loc: "BackupTarget"
  note: "Hardcoded enum `BackupTarget` with `System`, `Vscode` variants."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
