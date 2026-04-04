---
label: "refacts"
---

## Goal

Encode invariants at the boundaries so that invalid states (empty identities) are hard or impossible to express in the core domain model. The `Identity` model allows invalid states by relying on a runtime `is_configured` check instead of enforcing invariants via types.

## Current State

- `src/domain/identity.rs`: `Identity` struct allows creating instances with empty strings and provides an `is_configured` method, which is a structural debt because invalid states can be expressed.
- `src/domain/ports/identity_store.rs`: `IdentityState` holds `personal: Identity` and `work: Identity` directly, assuming they are always present, but they might not be configured.
- `src/adapters/identity_store.rs`: Adapts to `Identity` directly, and tests use empty/dummy strings.
- `src/app/commands/switch/mod.rs`: `switch::execute` manually checks if identity is configured and fails.
- `src/app/commands/identity/mod.rs`: Handles the creation of `Identity` without type-safe validation and falls back to empty default strings when loading the state if they do not exist.

## Plan

1. Update `src/domain/identity.rs` to enforce non-empty `name` and `email` upon construction via `TryFrom` (with a raw struct for deserialization) or `new`, and remove `is_configured()`.
2. Update `src/domain/ports/identity_store.rs` to change `IdentityState`'s `personal` and `work` fields to be `Option<Identity>`.
3. Update `src/adapters/identity_store.rs` to adjust dummy state generation and modify `get_identity` to return `Result<Option<Identity>, AppError>`.
4. Update `src/app/commands/switch/mod.rs` so that `switch::execute` handles `None` from `get_identity` directly instead of using `is_configured()`.
5. Update `src/app/commands/identity/mod.rs` so that `set` and `show` construct and handle `Identity` properly using `Option` and the new constructor.

## Constraints

- Code changes must adhere to single responsibility and accurate domain modeling.
- Modifications should not break unconnected tests.
- Residue checks for the `is_configured` method must be performed.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified.
