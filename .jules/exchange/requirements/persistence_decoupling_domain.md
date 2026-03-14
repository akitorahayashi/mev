---
label: "refacts"
implementation_ready: false
---

## Goal
Decouple core domain entities from specific persistence serialization libraries.

## Problem
Domain entities like `VcsIdentity` derive `serde::Serialize` and `serde::Deserialize`, which tightly couples the domain model to serialization constraints and specific formats (e.g., JSON). This leaks transport and persistence concerns into the core domain layer, violating Boundary Sovereignty.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "persistence_leak_data_arch.md"
  path: "src/domain/vcs_identity.rs"
  loc: "VcsIdentity"
  note: "Derives `serde::Serialize` and `serde::Deserialize` directly on the core domain model."
- source_event: "persistence_leak_data_arch.md"
  path: "src/domain/ports/identity_store.rs"
  loc: "IdentityState"
  note: "Derives `serde::Serialize` and `serde::Deserialize` directly on the domain port model."

## Change Scope
- `src/domain/vcs_identity.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store/local_json.rs`

## Constraints
- Serialization logic must strictly live inside the adapter layer (e.g. `local_json.rs`).

## Acceptance Criteria
- `serde` derivatives are removed from `src/domain/`.
- `local_json.rs` maps intermediate DTOs to/from the core domain entities.
