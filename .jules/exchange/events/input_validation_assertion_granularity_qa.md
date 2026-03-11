---
label: "tests"
created_at: "2026-03-11"
author_role: "qa"
confidence: "high"
---

## Problem

Input validation tests have poor diagnosability due to excessively broad assertions. Tests assert on generic strings rather than specific, structured errors.

## Goal

Improve test assertion granularity so failures clearly identify what constraint was violated, avoiding masked bugs.

## Context

In `tests/security/input_validation.rs`, assertions checking stderr contain very broad assertions like `predicate::str::contains("error").or(predicate::str::contains("Error"))`. This makes it impossible to know if the failure occurred due to the expected profile validation error, or a completely unrelated unexpected error during execution.

## Evidence

- path: "tests/security/input_validation.rs"
  loc: "14"
  note: "`create_rejects_invalid_profile` asserts on any string containing 'error', which masks failures like invalid configuration loading."

- path: "tests/security/input_validation.rs"
  loc: "25"
  note: "`switch_rejects_invalid_profile` uses the same non-specific assertion for an invalid profile."

## Change Scope

- `tests/security/input_validation.rs`
