---
label: "refacts"
---

## Goal
Minimize the public API surface area to enforce unidirectional dependency flow and prevent application layers from acting as generic domain type catalogs.

## Current State
- `src/app/api.rs`: Indiscriminately re-exports internal domain models (`BackupTarget`, `AppError`, `ExecutionPlan`, etc.) via `pub use crate::domain::...`. This entangles the application layer with domain specifics and presents internal architecture details through the public orchestration facade.
- `src/lib.rs`: Exports `app::cli::run as cli` and `app::api`, making `app` the public interface of the crate.

## Plan

1. Remove `pub use crate::domain::...` statements from `src/app/api.rs`.
2. Remove `pub use app::api;` from `src/lib.rs`.
3. Modify `src/lib.rs` to expose strict API edge contracts. (e.g. keeping `pub use app::cli::run as cli;` and removing the blanket `api` export).
4. Execute `just test` to verify no externally observable behavior is altered.

## Acceptance Criteria
- `pub use` statements for core domain objects are removed from `api.rs`.
- Only strict API edge contracts are exposed by `lib.rs`.
