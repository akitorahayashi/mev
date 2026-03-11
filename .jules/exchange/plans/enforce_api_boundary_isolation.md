---
label: "refacts"
---

## Goal

Map domain types to API-specific structs/enums instead of directly re-exporting them, ensuring strict boundary isolation.

## Problem

The API layer directly re-exports domain types instead of mapping them to API-specific structs/enums. This means internal domain changes could implicitly break external consumers, coupling internal logic to the public API contract.

## Affected Areas

### Application API
- `src/app/api.rs`

### Domain
- `src/domain/backup_target.rs`
- `src/domain/execution_plan.rs`
- `src/domain/ports/identity_store.rs`
- `src/domain/profile.rs`
- `src/domain/vcs_identity.rs`

## Constraints

- The API layer (`src/app/api.rs`) must map domain types to API-specific structs/enums instead of directly re-exporting them.

## Risks

- External consumers of `mev::api` could be broken if the API-specific types don't match the previously exported structures. The plan should create identical types within `api.rs` to maintain backward compatibility for external consumers.

## Acceptance Criteria

- `src/app/api.rs` no longer uses `pub use` to directly re-export domain entities.
- `src/app/api.rs` defines specific external-facing structs and uses From/Into traits or explicit mapping functions to map from the internal domain entities.

## Implementation Plan

1. Inspect `src/app/api.rs` to see the current domain re-exports.
2. In `src/app/api.rs`, remove the `pub use` statements for `BackupTarget`, `ExecutionPlan`, `IdentityState`, `Profile`, `SwitchIdentity`, and `VcsIdentity`.
3. In `src/app/api.rs`, define API-specific types to replace the re-exports (e.g., `pub enum Profile { Macbook, MacMini, Common }`).
4. In `src/app/api.rs`, implement `From` and `Into` traits to convert between the API types and the corresponding `crate::domain::*` types.
5. In `src/app/api.rs`, update the function signatures (`create`, `make`, `switch`, etc.) to accept and return the new API-specific types, using `.into()` to pass the domain equivalents to the internal `commands::*::execute` functions.
6. Verify changes using `list_files` and `read_file` to ensure `src/app/api.rs` contains the mapping logic and no longer directly re-exports domain types.
7. Run tests using `just test` to ensure the mappings are correct and haven't introduced regressions.
8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
