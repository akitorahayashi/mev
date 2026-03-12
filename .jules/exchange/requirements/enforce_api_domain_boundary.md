---
label: "refacts"
implementation_ready: false
---

## Goal

Isolate boundary domains by ensuring the API layer provides its own specific structs and enums mapped to internal domain types, instead of directly re-exporting them.

## Problem

The API layer (`src/app/api.rs`) directly re-exports internal domain types like `Profile` and `BackupTarget`. This couples the public API directly to the internal domain models, violating the boundary isolation rule and making domain evolution difficult without breaking external contracts.

## Evidence

- source_event: "api_domain_reexport_data_arch.md"
  path: "src/app/api.rs"
  loc: "pub use crate::domain::*"
  note: "Directly re-exports `BackupTarget`, `AppError`, `ExecutionPlan`, `IdentityState`, `Profile`, `SwitchIdentity`, and `VcsIdentity`."

## Change Scope

- `src/app/api.rs`

## Constraints

- The API layer must implement conversion functions (e.g., `From`/`Into`) between the new API types and internal domain types.

## Acceptance Criteria

- `src/app/api.rs` no longer directly re-exports domain models.
- API-specific structs and enums are created for public consumption.
- Data transfers between layers use proper mapping/conversion logic.
