---
label: "refacts"
---

## Goal

Rename `SwitchIdentity` to a domain-appropriate noun (`IdentityProfile`) to describe the concept rather than the CLI command action (`switch`).

## Current State

- `src/domain/vcs_identity.rs`: Contains the `SwitchIdentity` enum, its methods, aliases (`SWITCH_IDENTITY_ALIASES`), and resolver function (`resolve_switch_identity`). The name focuses on the command action (`switch`) instead of the domain context (identity profile).
- `src/domain/ports/identity_store.rs`: Uses `SwitchIdentity` in the `IdentityStore` trait method signature.
- `src/adapters/identity_store/local_json.rs`: Implements `IdentityStore` and uses `SwitchIdentity`.
- `src/app/api.rs`: Exposes `SwitchIdentity` in the API footprint for the `switch` command.
- `src/app/commands/switch/mod.rs`: Consumes `SwitchIdentity` during command execution.
- `src/app/cli/switch.rs`: Resolves CLI input using `resolve_switch_identity`.
- `tests/library/domain_exports.rs`: Tests public footprint by explicitly importing and testing `SwitchIdentity` and `resolve_switch_identity`.

## Plan

1. Rename `SwitchIdentity` to `IdentityProfile` in `src/domain/vcs_identity.rs`.
   - Update `SwitchIdentity` to `IdentityProfile`.
   - Update `SWITCH_IDENTITY_ALIASES` to `IDENTITY_PROFILE_ALIASES`.
   - Update `resolve_switch_identity` to `resolve_identity_profile`.
   - Update all references within the `tests` module in this file.
2. Update the domain ports boundary in `src/domain/ports/identity_store.rs` to use `IdentityProfile` in the `IdentityStore` trait definition.
3. Update the adapter implementation in `src/adapters/identity_store/local_json.rs` to use `IdentityProfile`.
4. Update the application layer in `src/app/api.rs`, `src/app/commands/switch/mod.rs`, and `src/app/cli/switch.rs` to use `IdentityProfile` and `resolve_identity_profile`.
5. Update external library tests in `tests/library/domain_exports.rs` to assert on `IdentityProfile` and `resolve_identity_profile` instead of the old names.
6. Run `cargo test` in the repository root to verify all changes and assure no broken tests or regressions.
7. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- `SwitchIdentity` is successfully renamed to `IdentityProfile` everywhere.
- `resolve_switch_identity` is renamed to `resolve_identity_profile`.
- `SWITCH_IDENTITY_ALIASES` is renamed to `IDENTITY_PROFILE_ALIASES`.
- The codebase complies with domain-driven design, avoiding naming domain models after command actions.
- All tests pass, proving correctness without regression.

## Risks

- Missed occurrences in documentation or test exports causing test suite failure. To mitigate, grep across the tree.
- Downstream crates or modules dependent on the old names could break. To mitigate, we rename all found usages and run `cargo test`.
