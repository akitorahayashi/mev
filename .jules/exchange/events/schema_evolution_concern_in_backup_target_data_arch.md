---
label: "refacts"
created_at: "2024-05-18"
author_role: "data_arch"
confidence: "high"
---

## Problem

Schema evolution risks when adding new variants to `BackupTarget`.

## Goal

Consolidate the definition of backup targets so that adding a new variant requires minimal modifications to scattered match arms.

## Context

Adding a new backup target requires changing multiple disparate locations in `BackupTarget`: adding a match arm in `from_input`, `name`, `description`, `role`, and adding it to the hardcoded array in `all()`. This violates the Single Source of Truth principle and is prone to errors during schema evolution (e.g., forgetting to add the new target to `all()`).

## Evidence

- path: "src/domain/backup_target.rs"
  loc: "14-49"
  note: "Adding a new target requires changes to `from_input`, `all()`, `name()`, `description()`, and `role()`."

- path: "src/domain/backup_target.rs"
  loc: "22-25"
  note: "Hardcoded array in `all()` requires manual maintenance and is out of sync with the enum definition."

## Change Scope

- `src/domain/backup_target.rs`
