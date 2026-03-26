---
label: "refacts"
implementation_ready: false
---

## Goal

Rename `SwitchIdentity` to a domain-appropriate noun such as `IdentityProfile` or `IdentityContext`.

## Problem

In `src/domain/vcs_identity.rs`, `SwitchIdentity` represents the profile of an identity (`Personal`, `Work`). Its name focuses on the command action (`switch`) rather than the domain concept.

## Evidence

- source_event: "switch_identity_taxonomy.md"
  path: "src/domain/vcs_identity.rs"
  loc: "enum SwitchIdentity"
  note: "Defines `SwitchIdentity` Enum which includes Personal and Work."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/adapters/identity_store/local_json.rs`

## Constraints

- The new name must describe the identity context and avoid colliding with existing CLI commands.

## Acceptance Criteria

- `SwitchIdentity` is renamed and references to it are updated.