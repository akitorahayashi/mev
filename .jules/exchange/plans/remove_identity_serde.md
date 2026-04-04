---
label: "refacts"
---

## Goal

Keep domain models independent of transport/persistence concerns by removing `serde` derivations from the core domain and handling serialization in the adapter layer.

## Current State

Persistence concerns (`serde::Serialize`, `serde::Deserialize`) are leaking into core domain models.
- `src/domain/identity.rs`: `Identity` struct derives `serde::Serialize` and `serde::Deserialize`.
- `src/domain/ports/identity_store.rs`: `IdentityState` struct derives `serde::Serialize` and `serde::Deserialize`.

## Plan

1. Remove `serde::Serialize` and `serde::Deserialize` derives from `Identity` in `src/domain/identity.rs`.
2. Move `IdentityState` from `src/domain/ports/identity_store.rs` to `src/adapters/identity_store.rs`, keeping its `serde` derives.
3. Update `src/adapters/identity_store.rs` to define its own DTOs for `Identity` and `IdentityState` if necessary, and implement conversion logic to and from the domain `Identity` model.
4. Update `IdentityStore` trait in `src/domain/ports/identity_store.rs` to no longer reference `IdentityState` (or modify it to use domain models directly, e.g. return `(Identity, Identity)` or a new domain structure that does not derive serde).
5. Ensure `src/adapters/identity_store.rs` handles all JSON serialization/deserialization internally and maps to the domain models before returning them to the caller.
6. Verify no `serde` imports or derives remain in `src/domain/` by running `rg serde src/domain/`.

## Constraints

- Code changes must adhere to single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.
- The `IdentityStore` adapter must continue storing JSON configuration correctly in the expected format.

## Acceptance Criteria

- `Identity` and other domain models in `src/domain/` do not derive `serde` traits.
- `IdentityState` and other DTOs used for persistence are localized to `src/adapters/`.
- All tests pass, including the identity store adapter tests.
