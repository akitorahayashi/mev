---
label: "refacts"
created_at: "2026-03-13"
author_role: "data_arch"
confidence: "high"
---

## Problem

Domain models are directly coupled to persistence serialization frameworks.

## Goal

Decouple domain models from persistence concerns by removing `serde` derivations and separating transport/storage types from core facts.

## Context

First Principles demand Boundary Sovereignty: keeping domain models independent of transport or runtime concerns. Currently, `VcsIdentity` and `IdentityState` derive `serde::Serialize` and `serde::Deserialize`, which couples core business rules to the JSON persistence format used by the local adapter.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "VcsIdentity"
  note: "Derives `serde::Serialize` and `serde::Deserialize`."
- path: "src/domain/ports/identity_store.rs"
  loc: "IdentityState"
  note: "A top-level model stored on disk deriving serde traits."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store/local_json.rs`
