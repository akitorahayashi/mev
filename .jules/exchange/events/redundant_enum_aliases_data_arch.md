---
label: "refacts"
created_at: "2024-05-23"
author_role: "data_arch"
confidence: "high"
---

## Problem

Domain models map user strings to enum variants using static alias arrays (`PROFILE_ALIASES`, `SWITCH_IDENTITY_ALIASES`, `BACKUP_TARGET_ALIASES`) that hardcode and duplicate the canonical string names and aliases already associated with the enums.

## Goal

Consolidate the string-to-variant mapping logic to use a single source of truth by dynamically checking variant canonical names and explicitly defined aliases, removing the redundant static arrays.

## Context

The Single Source of Truth principle dictates that each fact has one canonical representation. Hardcoding mapping arrays forces developers to update multiple places when adding new variants or aliases, increasing the risk of data drift and mapping inconsistencies.

## Evidence

- path: "src/domain/profile.rs"
  loc: "55-62"
  note: "`PROFILE_ALIASES` duplicates the canonical names from `as_str()` and aliases from `aliases()`."
- path: "src/domain/identity.rs"
  loc: "44-49"
  note: "`SWITCH_IDENTITY_ALIASES` hardcodes variants and strings redundantly."
- path: "src/domain/backup_target.rs"
  loc: "54-59"
  note: "`BACKUP_TARGET_ALIASES` duplicates the canonical names from `name()`."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/identity.rs`
- `src/domain/backup_target.rs`
