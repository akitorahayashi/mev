---
label: "refacts"
---

## Goal

Unify naming around profiles by replacing "machine" with "device" (e.g., `is_device_profile`, `validate_device_profile`).

## Current State

The term "profile" is used inconsistently. Functions use "machine profile" to mean any profile except `Global` (i.e. `Macbook` or `MacMini`). The CLI help text uses "Profile to create" or "Profile to use". This violates "One Concept, One Preferred Term".

- `src/domain/profile.rs`: Uses "machine profile" in documentation and function names (e.g. `is_machine_profile`, `validate_machine_profile`) instead of "device profile".
- `src/app/cli/create.rs`: Calls `validate_machine_profile` from `src/domain/profile.rs`.
- `docs/architecture.md`: Defines Profile as "A machine hardware configuration target" instead of "device".

## Plan

1. Rename functions and strings in `src/domain/profile.rs` using `sed` or by writing the file.
   - Rename `is_machine_profile` to `is_device_profile`
   - Rename `validate_machine_profile` to `validate_device_profile`
   - Update error messages to use "device profile" instead of "machine profile"
   - Update `#[test]` function names to use `device` instead of `machine`.
2. Update callers in `src/app/cli/create.rs` using `sed` or by writing the file.
   - Call `validate_device_profile` instead of `validate_machine_profile`.
3. Update `docs/architecture.md`
   - Update the `Profile` definition to use "device hardware configuration target" instead of "machine hardware configuration target".
4. Run tests (`cargo test` and `cd crates/mev-internal && cargo test`) to ensure no regressions were introduced.
5. Search for remnants of "machine profile" or "machine_profile" to ensure no missed occurrences.

## Acceptance Criteria

- `machine` functions in `profile.rs` are renamed to use `device` consistently.
- `create.rs` uses the newly renamed function.
- `docs/architecture.md` describes a Profile as a device target.
- Tests pass.
