---
label: "refacts"
created_at: "2026-04-17"
author_role: "modeler"
confidence: "high"
---

## Problem

Transport/Persistence concerns (Serde macros and attributes) are leaking into the core domain model (`Identity`, `RawIdentity`, `IdentityState`), violating the Boundary Sovereignty principle.

## Goal

Remove `serde` dependencies and serialization concepts from the `src/domain/` layer. Serialization types should belong entirely to the `src/adapters/` layer (e.g., `src/adapters/identity_store.rs`), preserving the core domain models as pure facts.

## Context

The Boundary Sovereignty design rule dictates that core domain models must be kept independent of transport, persistence, UI, or runtime concerns. Currently, `src/domain/identity.rs` and `src/domain/ports/identity_store.rs` directly implement `serde::Serialize` and `serde::Deserialize` and use attributes like `#[serde(try_from)]` or `#[serde(deserialize_with)]`. This couples the pure domain rules of Identity to JSON on-disk persistence formats.

## Evidence

- path: "src/domain/identity.rs"
  loc: "10"
  note: "`RawIdentity` derives `serde::Serialize` and `serde::Deserialize`"
- path: "src/domain/identity.rs"
  loc: "16-17"
  note: "`Identity` derives `serde::Serialize` and `serde::Deserialize`, and relies on `#[serde(try_from = "RawIdentity", into = "RawIdentity")]`."
- path: "src/domain/ports/identity_store.rs"
  loc: "25-30"
  note: "`IdentityState` derives `serde::Serialize` and `serde::Deserialize` and applies `deserialize_with` attributes to its fields."

## Change Scope

- `src/domain/identity.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store.rs`
