---
label: "tests"
implementation_ready: false
---

## Goal

Increase test coverage across critical domain and adapter paths to exceed the 40% threshold, ensuring regression risks are mitigated and safety gates pass in CI/CD. Ensure key domain entities are unit tested for valid state transitions and boundaries.

## Problem

Code coverage is around 21%, which is below the 40.00% failure threshold configured in the `just coverage` target. Critical domain transitions and decision paths in `repo_target.rs` and `repository_ref.rs` have very low line coverage (currently 0 lines covered according to tarpaulin report because test assertions are failing to execute or cover all paths), creating high regression risk in core routing and identification logic. Adapter components (`gh`, `git`, `process`) also have missing coverage.

## Evidence

- source_event: "coverage_below_threshold_cov.md"
  path: "justfile"
  loc: "line 77"
  note: "Specifies --fail-under 40 for cargo tarpaulin."
- source_event: "critical_domain_logic_untested_cov.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "Entire file"
  note: "0/48 lines tested. Untested logic paths in tarpaulin report."
- source_event: "critical_domain_logic_untested_cov.md"
  path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "Entire file"
  note: "0/7 lines tested. Domain entities are fundamentally untested in tarpaulin report."
- source_event: "coverage_below_threshold_cov.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "Entire file"
  note: "Coverage Results indicate 0/25 lines tested."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/adapters/git.rs`

## Constraints

- Domain logic tests must reside as self-contained unit tests within their respective `src/domain/` modules inside a `#[cfg(test)]` block.
- Filesystem and I/O boundaries in tests must be strictly isolated using temporary directories or dependency injection.
- Tests must be self-contained with no reliance on external services or mutable shared state.

## Acceptance Criteria

- `just coverage` passes, achieving at least 40.00% coverage overall.
- `crates/mev-internal/src/domain/repository_ref.rs` contains unit tests for its core logic.
- `crates/mev-internal/src/domain/repo_target.rs` contains unit tests for its core logic.
