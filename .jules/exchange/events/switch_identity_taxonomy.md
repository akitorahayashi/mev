---
label: "refacts"
created_at: "2024-05-18"
author_role: "taxonomy"
confidence: "high"
---

## Problem

In `src/domain/vcs_identity.rs`, there is a struct `VcsIdentity` (name, email) and an enum `SwitchIdentity` (`Personal`, `Work`). `SwitchIdentity` represents the profile or category of the identity, but naming it `SwitchIdentity` focuses on the command action (`switch`) rather than the domain concept. It's essentially an `IdentityProfile` or `IdentityContext`. This violates the "Names Are Interfaces" and "Domain Language First" principles by naming the domain concept after a CLI command action.

## Goal

Rename `SwitchIdentity` to a domain-appropriate noun that doesn't couple the type to the `switch` command. A good candidate is `IdentityProfile`, `IdentityContext`, or simply `IdentityKind`. Let's choose `IdentityContext` or `IdentityProfile` (given "profile" is a known repository term, "context" or "kind" might be safer to avoid collision).

## Context

The enum `SwitchIdentity` defines the available identities (Personal, Work), but its name is tied to the verb "switch", making it awkward to use outside of that command (e.g. if we want to list or validate an identity kind).

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "enum SwitchIdentity"
  note: "Defines `SwitchIdentity` Enum which includes Personal and Work"
- path: "src/adapters/identity_store/local_json.rs"
  loc: "fn get_identity"
  note: "Method uses `SwitchIdentity` in store interface."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/adapters/identity_store/local_json.rs`