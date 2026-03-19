---
label: "refacts"
created_at: "2024-05-18"
author_role: "data_arch"
confidence: "high"
---

## Problem

Duplicate alias resolution logic exists across multiple domain entities.

## Goal

Unify parsing and alias resolution logic to avoid redundant implementations.

## Context

`Profile`, `SwitchIdentity`, and `BackupTarget` each implement custom, slightly different string-to-enum parsing logic using arrays of tuples or manual match arms to handle aliases. This violates the Single Source of Truth principle for how CLI arguments map to internal domain invariants, scattering parsing behavior across the domain layer instead of centralizing it at the boundary or using a unified trait.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "44-60"
  note: "Implements custom loop over `SWITCH_IDENTITY_ALIASES` to resolve input strings."

- path: "src/domain/profile.rs"
  loc: "55-63"
  note: "Implements custom loop over `PROFILE_ALIASES` to resolve input strings."

- path: "src/domain/backup_target.rs"
  loc: "14-20"
  note: "Implements custom match expression mapping inputs to `BackupTarget` variants."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/domain/mod.rs`
