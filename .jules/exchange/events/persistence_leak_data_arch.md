---
label: "refacts"
created_at: "2024-03-14"
author_role: "data_arch"
confidence: "high"
---

## Problem

Core domain entities derive `serde::Serialize` and `serde::Deserialize`, tightly coupling the domain model to serialization mechanisms and formats (e.g., JSON).

## Goal

Decouple domain entities from persistence concerns. Serialization and deserialization should be handled by DTOs in the adapter layer (e.g., in `src/adapters/identity_store/local_json.rs`), which are then mapped to the core domain types.

## Context

Boundary Sovereignty mandates that domain models must be independent of transport or persistence concerns. Deriving `serde` traits in the domain layer allows persistence details to leak into core logic, violating this principle.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "VcsIdentity"
  note: "Derives `serde::Serialize` and `serde::Deserialize` directly on the core domain model."
- path: "src/domain/ports/identity_store.rs"
  loc: "IdentityState"
  note: "Derives `serde::Serialize` and `serde::Deserialize` directly on the domain port model."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store/local_json.rs`
