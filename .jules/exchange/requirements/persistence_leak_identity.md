---
label: "refacts"
implementation_ready: true
---

## Goal

Keep domain models independent of transport/persistence concerns by removing `serde` derivations from the core domain and handling serialization in the adapter layer.

## Problem

Persistence concerns (`serde::Serialize`, `serde::Deserialize`) are leaking into core domain models (`Identity` and `IdentityState`).

## Context

First Principles dictate "Boundary Sovereignty: keep domain models independent of transport/UI/runtime concerns." The `Identity` struct and `IdentityState` struct have `#[derive(..., serde::Serialize, serde::Deserialize)]`. Anti-patterns include "Transport DTOs or persistence types leaking into core domain logic."

## Evidence

- path: "src/domain/identity.rs"
  loc: "Identity struct"
  note: "`Identity` struct derives `serde::Serialize` and `serde::Deserialize`."
- path: "src/domain/ports/identity_store.rs"
  loc: "IdentityState struct"
  note: "`IdentityState` struct derives `serde::Serialize` and `serde::Deserialize`."
- path: "src/adapters/identity_store.rs"
  loc: "29-39"
  note: "Adapter uses these traits for JSON persistence."

## Change Scope

- `src/domain/identity.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
