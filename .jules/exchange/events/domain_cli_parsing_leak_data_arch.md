---
label: "refacts"
created_at: "2026-03-13"
author_role: "data_arch"
confidence: "high"
---

## Problem

Domain models contain CLI-specific string input parsing logic and aliases.

## Goal

Move input alias definition and resolution logic out of domain models and into the CLI adapter layer where it belongs.

## Context

Boundary Sovereignty dictates that domain models must not handle transport/UI concerns. Currently, `Profile`, `SwitchIdentity`, and `tag` models define CLI string aliases (e.g., "mbk", "p", "rust" expanding to "rust-platform") and the logic to parse user strings into domain types. This validation and mapping should be handled by the adapter/CLI boundary.

## Evidence

- path: "src/domain/profile.rs"
  loc: "Profile::aliases"
  note: "Defines CLI aliases like 'mbk' for 'macbook'."
- path: "src/domain/profile.rs"
  loc: "resolve_profile"
  note: "Parses CLI input strings into Profile types."
- path: "src/domain/vcs_identity.rs"
  loc: "resolve_switch_identity"
  note: "Parses CLI input strings into SwitchIdentity types."
- path: "src/domain/tag.rs"
  loc: "tag_groups"
  note: "Defines CLI tag expansion groups."
- path: "src/domain/tag.rs"
  loc: "resolve_tags"
  note: "Expands CLI input strings into concrete tags."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/vcs_identity.rs`
- `src/domain/tag.rs`
- `src/app/cli/make.rs`
- `src/app/cli/switch.rs`
- `src/app/cli/create.rs`
