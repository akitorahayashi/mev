---
label: "tests"
implementation_ready: false
---

## Goal
Ensure core domain invariants (`ExecutionPlan` and `RepositoryRef`) have complete isolated unit test coverage.

## Problem
Crucial domain boundaries lack fundamental testing. `ExecutionPlan` constructs deployment plans but has no `#[cfg(test)]` module validating its internal tagging logic. `RepositoryRef` contains parsing logic for Git remotes (SSH, HTTPS) but shows 0% coverage during workspace test runs, exposing critical logic to silent regressions.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "missing_execution_plan_tests_qa.md"
  path: "src/domain/execution_plan.rs"
  loc: "12-22"
  note: "`ExecutionPlan::full_setup` and `ExecutionPlan::make` contain pure plan construction logic but have no tests."
- source_event: "repository_ref_parsing_uncovered_cov.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "3-83"
  note: "Line coverage is at 0/48 lines. Unexecuted code includes `parse_scp_like_remote` and `parse_ssh_remote`."

## Change Scope
- `src/domain/execution_plan.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`

## Constraints
- Tests must be pure unit tests without external side effects.

## Acceptance Criteria
- `ExecutionPlan` tagging variations are validated by unit tests.
- `RepositoryRef` parsing functions achieve 100% logic path coverage.
