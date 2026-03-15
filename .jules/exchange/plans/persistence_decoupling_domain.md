---
label: "refacts"
---

## Goal

Decouple core domain entities (`VcsIdentity`, `IdentityState`) from specific persistence serialization libraries (`serde`) by isolating serialization logic inside the adapter layer (`local_json.rs`).

## Current State

- `src/domain/vcs_identity.rs`: `VcsIdentity` derives `serde::Serialize` and `serde::Deserialize`, which tightly couples the domain model to serialization constraints, leaking transport and persistence concerns into the core domain layer, violating Boundary Sovereignty.
- `src/domain/ports/identity_store.rs`: `IdentityState` derives `serde::Serialize` and `serde::Deserialize`, which tightly couples the domain port model to serialization constraints, leaking transport and persistence concerns into the core domain layer, violating Boundary Sovereignty.
- `src/adapters/identity_store/local_json.rs`: Directly uses domain structs for serialization/deserialization with `serde_json`, failing to maintain an independent internal schema mapping.

## Plan

1. Remove `serde::Serialize` and `serde::Deserialize` derives from `src/domain/vcs_identity.rs` using `replace_with_git_merge_diff`.
2. Verify removal of serde derives from `src/domain/vcs_identity.rs` by reading the file.
3. Remove `serde::Serialize` and `serde::Deserialize` derives from `src/domain/ports/identity_store.rs` using `replace_with_git_merge_diff`.
4. Verify removal of serde derives from `src/domain/ports/identity_store.rs` by reading the file.
5. In `src/adapters/identity_store/local_json.rs`, use `replace_with_git_merge_diff` to:
   - Introduce Data Transfer Objects (DTOs) `VcsIdentityDto` and `IdentityStateDto` that derive `serde::Serialize` and `serde::Deserialize`.
   - Implement `From<IdentityStateDto> for IdentityState` (and `From<VcsIdentityDto> for VcsIdentity`).
   - Implement `From<&IdentityState> for IdentityStateDto` (and `From<&VcsIdentity> for VcsIdentityDto`).
   - Update `IdentityFileStore::load` to deserialize into `IdentityStateDto` and convert to `IdentityState`.
   - Update `IdentityFileStore::save` to convert from `IdentityState` to `IdentityStateDto` and serialize.
6. Verify modifications in `src/adapters/identity_store/local_json.rs` via `git diff`.
7. Run `cargo check` and `just test` to ensure tests and compilation pass.
8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- `serde` derivations (`Serialize`, `Deserialize`) are completely removed from `src/domain/vcs_identity.rs` and `src/domain/ports/identity_store.rs`.
- `local_json.rs` defines its own DTOs with serialization support and maps intermediate DTOs to/from the core domain entities.
- The externally observable behavior of loading and saving identities is unchanged.

## Risks

- Deserializing existing `.identity.json` and `config.json` fails if the new DTO structures do not exactly match the previous serialized format. Mitigated by ensuring DTOs maintain identical fields to the original domain structs.
