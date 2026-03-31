---
label: "tests"
implementation_ready: false
---

## Goal

Ensure that all methods of the `IdentityFileStore` adapter (`exists`, `load`, `save`, and `get_identity`) are fully tested.

## Problem

The JSON identity store implementation in `src/adapters/identity_store/local_json.rs` has very low test coverage and is missing critical path assertions, especially around persistence and error handling.

## Context

The `IdentityFileStore` manages reading and writing the user's identities (`personal` and `work`) to disk. This involves reading legacy locations, atomic writes via temporary files, and parsing JSON state. The coverage report (generated via default `cargo tarpaulin` configuration) indicates that the logic for file loading, atomic writing, and fetching specific identities is not completely covered by tests according to the line coverage metric. Untested code in an identity configuration adapter risks data loss or corruption during save operations.

## Evidence

- path: "src/adapters/identity_store/local_json.rs"
  loc: "load, save, get_identity"
  note: "Untested paths include legacy path reading/migration, atomic save steps (temporary file creation, serialization error handling, renaming and cleanup), and retrieving specific identities."

## Change Scope

- `src/adapters/identity_store/local_json.rs`

## Constraints

- Use temporary files to avoid polluting local system state.

## Acceptance Criteria

- Tests verify legacy migration, atomic saves, and specific identity retrieval in `IdentityFileStore`.
