---
label: "refacts"
created_at: "2024-03-14"
author_role: "data_arch"
confidence: "high"
---

## Problem

Core domain models contain CLI-specific string input parsing logic and hardcoded alias mappings.

## Goal

Relocate string input parsing and alias resolution to the adapter or application CLI layer, preserving the domain models' focus solely on business rules and valid internal states.

## Context

Architecture Rule (Domain Input Parsing) dictates that core domain models must not contain CLI-specific string input parsing logic or aliases. Validation, UI mapping, and string parsing must be exclusively handled by the adapter or application CLI layer to maintain Boundary Sovereignty.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "SWITCH_IDENTITY_ALIASES"
  note: "Defines hardcoded CLI aliases within the domain model."
- path: "src/domain/profile.rs"
  loc: "PROFILE_ALIASES"
  note: "Contains CLI aliases and mapping rules for profile resolution."
- path: "src/domain/backup_target.rs"
  loc: "from_input"
  note: "Implements string-based parsing and alias resolution directly on the domain type."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/`
