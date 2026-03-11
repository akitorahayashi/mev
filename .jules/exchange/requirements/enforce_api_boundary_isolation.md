---
label: "refacts"
implementation_ready: false
---

## Goal

Map domain types to API-specific structs/enums instead of directly re-exporting them, ensuring strict boundary isolation.

## Problem

The API layer directly re-exports domain types instead of mapping them to API-specific structs/enums. This means internal domain changes could implicitly break external consumers, coupling internal logic to the public API contract.

## Evidence

- source_event: "api_boundary_isolation_structural_arch.md"
  path: "src/app/api.rs"
  loc: "14-19"
  note: "Direct `pub use` exports of `BackupTarget`, `ExecutionPlan`, `IdentityState`, `Profile`, `SwitchIdentity`, and `VcsIdentity` expose domain types directly to the API consumers."

## Change Scope

- `src/app/api.rs`
- `src/domain/backup_target.rs`
- `src/domain/execution_plan.rs`
- `src/domain/ports/identity_store.rs`
- `src/domain/profile.rs`
- `src/domain/vcs_identity.rs`

## Constraints

- The API layer (`src/app/api.rs`) must map domain types to API-specific structs/enums instead of directly re-exporting them.

## Acceptance Criteria

- `src/app/api.rs` no longer uses `pub use` to directly re-export domain entities.
- `src/app/api.rs` defines specific external-facing structs and uses From/Into traits or explicit mapping functions to map from the internal domain entities.
