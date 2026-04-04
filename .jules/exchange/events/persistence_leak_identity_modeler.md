---
label: "refacts"
created_at: "2024-10-24"
author_role: "modeler"
confidence: "high"
---

## Problem

Persistence concerns (`serde::Serialize`, `serde::Deserialize`) are leaking into core domain models (`Identity` and `IdentityState`).

## Goal

Keep domain models independent of transport/persistence concerns by removing `serde` derivations from the core domain and handling serialization in the adapter layer.

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
