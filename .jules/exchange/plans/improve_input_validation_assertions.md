---
label: "tests"
---

## Goal

Improve assertion granularity for security input validations.

## Problem

Input validation tests use excessively broad assertions (e.g., checking if stderr contains generic terms like "error"), which mask underlying bugs.

## Affected Areas

### Security Tests

- `tests/security/input_validation.rs`

## Constraints

- Tests must assert externally observable behavior at the owning boundary.

## Risks

- Tests might become brittle if error messages change frequently.

## Acceptance Criteria

- Assertions strictly assert on exact error sub-strings denoting the exact validation failure.

## Implementation Plan

1. In `tests/security/input_validation.rs`, update `create_rejects_invalid_profile` test. Change `predicate::str::contains("error").or(predicate::str::contains("Error"))` to `predicate::str::contains("invalid profile: nonexistent")`.
2. Update `switch_rejects_invalid_profile` test. Change the assertion to `predicate::str::contains("invalid identity: invalid identity 'badprofile'")`.
3. Run `cargo test` to verify the tests pass.
