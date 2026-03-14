---
label: "refacts"
implementation_ready: false
---

## Goal
Minimize the public API surface area to enforce unidirectional dependency flow and prevent application layers from acting as generic domain type catalogs.

## Problem
The `src/app/api.rs` and `src/lib.rs` files indiscriminately re-export internal domain models and components via `pub use`. This entangles the application layer with domain specifics, creating a backward dependency where internal architecture details are presented through the public orchestration facade.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "pub_use_flow_structural_arch.md"
  path: "src/app/api.rs"
  loc: "14-19"
  note: "Re-exports domain models (`BackupTarget`, `AppError`, `ExecutionPlan`, etc.) via `pub use crate::domain::...`."
- source_event: "pub_use_flow_structural_arch.md"
  path: "src/lib.rs"
  loc: "15-18"
  note: "Exports `app::cli::run as cli` and `app::api`. This makes `app` the public interface of the crate."

## Change Scope
- `src/app/api.rs`
- `src/lib.rs`

## Constraints
- The domain layer should not be re-exported by the application layer.

## Acceptance Criteria
- `pub use` statements for core domain objects are removed from `api.rs`.
- Only strict API edge contracts are exposed by `lib.rs`.
