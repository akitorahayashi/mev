---
label: "refacts"
implementation_ready: false
---

## Goal

Unify naming around profiles by replacing "machine" with "device" (e.g., `is_device`, `validate_device`).

## Problem

The term "profile" is used inconsistently. Functions use "machine profile" to mean any profile except `Global` (i.e. `Macbook` or `MacMini`). The CLI help text uses "Profile to create" or "Profile to use". This violates "One Concept, One Preferred Term".

## Evidence

- source_event: "machine_profile_taxonomy.md"
  path: "src/domain/profile.rs"
  loc: "fn validate_machine_profile"
  note: "Uses 'machine_profile' in function names like `is_machine_profile`."

## Change Scope

- `src/domain/profile.rs`
- `src/app/cli/create.rs`

## Constraints

- Differentiate between a global profile and a physical machine profile by using "device" instead of "machine".

## Acceptance Criteria

- `machine` functions in `profile.rs` are renamed to use `device` consistently.