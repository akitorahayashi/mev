---
label: "refacts"
---

## Goal

Establish a consistent naming convention and structural taxonomy for domain entities that act as targets or parameters for various system operations, specifically migrating away from CLI-action-named types to structurally correct domain nouns (`IdentityScope` and `BackupComponent`).

## Current State

The repository currently violates domain-language-first principles by naming domain models after the CLI commands that use them. This distributes CLI concerns into the core domain layer.

- `src/domain/identity.rs`: Defines `SwitchIdentity` which is named directly after the "switch" command. This must be structurally renamed to `IdentityScope` to reflect its domain purpose (e.g., Personal vs Work scope) rather than its usage context.
- `src/domain/backup_target.rs`: Defines `BackupTarget`, which improperly pairs the command name ("backup") with an overloaded, ambiguous generic term ("target"). This entire module boundary must be re-owned as `src/domain/backup_component.rs` providing `BackupComponent`.
- `src/domain/mod.rs` and `src/domain/error.rs`: Serve as domain aggregation points and boundary definitions that currently expose the legacy action-based terms `InvalidIdentity` and `InvalidBackupTarget`, which leak into upper layers.
- `src/app/commands/switch/mod.rs`, `src/app/commands/backup/mod.rs`, `src/app/cli/switch.rs`, `src/app/cli/backup.rs`, and `src/app/api.rs`: The application and CLI orchestration layers act as clients consuming the legacy domain structures. They currently expect action-named parameters rather than domain nouns.
- `src/domain/ports/identity_store.rs` and `src/adapters/identity_store/local_json.rs`: Configuration boundaries depend on the legacy `SwitchIdentity` parameter type.
- `tests/library/domain_exports.rs`, `tests/security/input_validation.rs`, and `tests/cli/identity.rs`: These tests assert on the legacy type names and specifically the verbatim string formats of domain errors (e.g., "invalid identity 'badprofile'"), coupling test assertions to the incorrect legacy vocabulary.

## Plan

1. Re-own the backup domain boundary by moving `src/domain/backup_target.rs` to `src/domain/backup_component.rs`. Update the `src/domain/mod.rs` manifest to expose the new boundary.
2. Refactor `src/domain/backup_component.rs` to replace the `BackupTarget` domain model with `BackupComponent`. Update all internal resolution and validation logic, constants, and display outputs to reflect the new component taxonomy instead of the generic target taxonomy.
3. Refactor `src/domain/identity.rs` to replace the `SwitchIdentity` domain model with `IdentityScope`. Update all internal resolution functions and aliases to reflect the scope taxonomy.
4. Redefine domain boundary errors in `src/domain/error.rs` by replacing `InvalidBackupTarget` with `InvalidBackupComponent` and `InvalidIdentity` with `InvalidIdentityScope`. Update the `Display` implementations to surface the new domain terms.
5. Update all downstream CLI and API consumers (`src/app/api.rs`, `src/app/commands/switch/mod.rs`, `src/app/commands/backup/mod.rs`, `src/app/cli/switch.rs`, `src/app/cli/backup.rs`) to depend exclusively on the new `BackupComponent` and `IdentityScope` domain nouns. The CLI input structure itself must remain unchanged (e.g., users still type `mev backup system`), but internal variables and function signatures must align with the new taxonomy.
6. Update configuration storage boundaries (`src/domain/ports/identity_store.rs`, `src/adapters/identity_store/local_json.rs`) to consume the updated `IdentityScope` parameter.
7. Update test boundaries (`tests/library/domain_exports.rs`, `tests/security/input_validation.rs`) to assert on the new structurally correct exports and error display strings. Specifically, ensure the test suite captures the shift from "invalid identity" to "invalid identity scope" in user-facing error output.
8. Execute standard workspace tests (`cargo test`) to ensure cross-boundary compilation and that externally observable behaviors (CLI inputs) are preserved while the internal domain language is corrected.
9. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
10. Submit the finalized changes.

## Acceptance Criteria

- The `src/domain/backup_target.rs` module is entirely removed and replaced by `src/domain/backup_component.rs`.
- The codebase contains no references to `SwitchIdentity` or `BackupTarget`.
- Upper layers depend exclusively on the new `IdentityScope` and `BackupComponent` domain nouns.
- Externally observable behavior remains unchanged: users can still execute `mev switch work` and `mev backup system`.
- All tests compile and pass with the updated structural assertions.

## Risks

- Incomplete re-ownership: Legacy aliases or internal variables might retain the old `SwitchIdentity` or `BackupTarget` names, silently preserving technical debt. This must be mitigated by a comprehensive remnant search before finalizing the structural transition.
- Unintentional CLI changes: Refactoring the internal domain names could accidentally leak into the CLI argument parsing definitions in `src/app/cli/backup.rs` or `src/app/cli/switch.rs`, altering the public user contract. The CLI flag inputs must remain stable while the internal mappings change.
