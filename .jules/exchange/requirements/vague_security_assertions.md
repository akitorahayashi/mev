---
label: "tests"
implementation_ready: true
---

## Goal
Refine overly broad test assertions in security tests to improve diagnosability.

## Problem
Security tests for input validation assert against broad string matches ("error" or "Error"), which fail to diagnose the root cause and permit false positives if unrelated failures occur.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "vague_security_assertions_qa.md"
  path: "tests/security/input_validation.rs"
  loc: "13, 25"
  note: "`stderr(predicate::str::contains(\"error\").or(predicate::str::contains(\"Error\")));` is overly broad in validation tests."

## Change Scope
- `tests/security/input_validation.rs`

## Constraints
- Security assertions must verify distinct, typed error outputs rather than generic "error" strings.

## Acceptance Criteria
- Security assertions in tests are updated to assert on specific error types or precise failure strings.
