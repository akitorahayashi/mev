---
label: "refacts"
created_at: "2024-05-18"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term "profile" is used inconsistently. In `src/domain/profile.rs`, functions like `validate_machine_profile` and `is_machine_profile` introduce the term "machine" as a modifier for "profile" to mean any profile except `Global` (i.e., `Macbook` or `MacMini`). This creates a new concept ("machine profile") that isn't cleanly separated from the domain concept of "profile" and isn't reflected in the user-facing CLI terminology, violating the "One Concept, One Preferred Term" and "Domain Language First" principles.

Furthermore, the CLI help text for `create` uses "Profile to create" whereas the `make` command uses "Profile to use", while the internal enum is just `Profile` representing `Global`, `Macbook`, `MacMini`. Using "machine" to differentiate these profiles internally creates an inconsistent dialect.

## Goal

Unify the naming around the concept of a profile. Rename `is_machine_profile` and `validate_machine_profile` to use a more consistent domain term, such as `is_device` and `validate_device` (or `validate_device_profile`), or just remove "machine" and use "device" instead. A "device" represents a physical machine, which correctly contrasts with the "global" profile.

## Context

The `create` command can only run against a concrete device profile (e.g. `macbook`, `mac-mini`), whereas the `make` command can run against `global` or a device profile. The distinction between a global profile and a device profile is important, but "machine" is an incidental implementation detail that should be replaced with "device" to better align with the concept of a physical target.

## Evidence

- path: "src/domain/profile.rs"
  loc: "fn validate_machine_profile"
  note: "Uses 'machine_profile' in function names like `is_machine_profile` and `validate_machine_profile`."

- path: "src/app/cli/create.rs"
  loc: "fn run"
  note: "Calls `profile::validate_machine_profile` to ensure the profile is not global."

## Change Scope

- `src/domain/profile.rs`
- `src/app/cli/create.rs`