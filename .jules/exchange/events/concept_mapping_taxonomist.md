---
label: "refacts"
created_at: "2024-04-17"
author_role: "taxonomist"
confidence: "high"
---

## Problem

The codebase employs inconsistent patterns for defining alias mappings for enums across the domain layer, resulting in duplicated and diverging logic to resolve aliases and canonical names for core concepts (`Profile`, `IdentityScope`, `BackupComponent`).

## Goal

Standardize how input string aliases map to enum variants by adopting a single, consistent convention across all domain concepts to improve comprehension and refactor safety.

## Context

Different domain entities handle input resolution differently. `Profile` and `IdentityScope` embed alias definitions as an instance method returning an array of strings (`aliases() -> &'static [&'static str]`). In contrast, `BackupComponent` uses a separate flat tuple array (`BACKUP_COMPONENT_ALIASES: &[(&str, BackupComponent)]`). These different shapes increase cognitive load for a simple, repetitive concept (string-to-enum parsing). Resolving logic also diverges between `resolve_profile` (using `.iter().find()`) and `resolve_backup_component` (using an explicit `for` loop over the custom array format).

## Evidence

- path: "src/domain/profile.rs"
  loc: "19-27"
  note: "Defines aliases via a method returning `&'static [&'static str]` and uses `.iter().find()` for resolution."

- path: "src/domain/identity.rs"
  loc: "47-52"
  note: "Defines aliases via a method returning `&'static [&'static str]` and uses `.iter().find()` for resolution."

- path: "src/domain/backup_component.rs"
  loc: "46-51"
  note: "Defines aliases via a separate `BACKUP_COMPONENT_ALIASES` const array mapping `&str` to `BackupComponent`."

- path: "src/domain/backup_component.rs"
  loc: "55-62"
  note: "Uses a procedural `for` loop to resolve the component instead of the functional mapping used in the other modules."

## Change Scope

- `src/domain/backup_component.rs`
