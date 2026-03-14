---
label: "tests"
created_at: "2026-03-14"
author_role: "qa"
confidence: "high"
---

## Problem

Security contract tests assert on broad stderr output ("error" or "Error") rather than specific diagnostic failure modes, severely reducing failure diagnosability.

## Goal

Update assertions to verify specific typed error messages or exit codes to ensure tests fail for the right reasons.

## Context

Broad string-matching assertions are anti-patterns because they can produce false positives. For example, if the CLI panics or fails for an unrelated configuration reason (like a missing dependency), it will also output "Error", and the test would mistakenly pass as having rejected the invalid profile.

## Evidence

- path: "tests/security/input_validation.rs"
  loc: "13"
  note: "`stderr(predicate::str::contains(\"error\").or(predicate::str::contains(\"Error\")));` is overly broad in `create_rejects_invalid_profile`."
- path: "tests/security/input_validation.rs"
  loc: "25"
  note: "`stderr(predicate::str::contains(\"error\").or(predicate::str::contains(\"Error\")));` is overly broad in `switch_rejects_invalid_profile`."

## Change Scope

- `tests/security/input_validation.rs`
