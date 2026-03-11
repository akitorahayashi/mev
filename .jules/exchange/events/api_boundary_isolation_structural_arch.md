---
label: "refacts"
created_at: "2026-03-11"
author_role: "structural_arch"
confidence: "high"
---

## Problem

The API layer directly re-exports domain types instead of mapping them to API-specific structs/enums.

## Goal

Map domain types to API-specific structs/enums instead of directly re-exporting them, ensuring strict boundary isolation.

## Context

The repository architecture enforces a strict boundary isolation where the API layer must not expose domain-internal implementations directly. Instead, domain types must be mapped to specific API data transfer objects. Directly exporting domain types means internal domain changes could implicitly break external consumers, coupling internal logic to the public API contract.

## Evidence

- path: "src/app/api.rs"
  loc: "14-19"
  note: "Direct `pub use` exports of `BackupTarget`, `ExecutionPlan`, `IdentityState`, `Profile`, `SwitchIdentity`, and `VcsIdentity` expose domain types directly to the API consumers."

## Change Scope

- `src/app/api.rs`
- `src/domain/backup_target.rs`
- `src/domain/execution_plan.rs`
- `src/domain/ports/identity_store.rs`
- `src/domain/profile.rs`
- `src/domain/vcs_identity.rs`
