---
label: "refacts"
created_at: "2024-05-24"
author_role: "data_arch"
confidence: "high"
---

## Problem

The API layer (`src/app/api.rs`) directly re-exports domain types like `Profile`, `BackupTarget`, `ExecutionPlan`, `IdentityState`, and `VcsIdentity` instead of mapping them to API-specific structs/enums.

## Goal

Isolate boundary domains by ensuring the API layer provides its own specific structs and enums that map to/from internal domain types, fulfilling the strict boundary isolation architecture rule.

## Context

Directly exposing domain structs in the API layer violates Boundary Sovereignty. The API represents the transport/UI boundary and should not leak internal domain models. This coupling makes it difficult to evolve the internal domain without breaking external API contracts.

## Evidence

- path: "src/app/api.rs"
  loc: "pub use crate::domain::*"
  note: "Directly re-exports `BackupTarget`, `AppError`, `ExecutionPlan`, `IdentityState`, `Profile`, `SwitchIdentity`, and `VcsIdentity`."

## Change Scope

- `src/app/api.rs`
