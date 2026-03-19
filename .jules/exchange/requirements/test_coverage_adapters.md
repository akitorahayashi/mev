---
label: "tests"
implementation_ready: false
---

## Goal

Provide test coverage for external integrations (GitHub, Git, Process) to assure resilient responses, error catching on external system failures, and proper boundary transformations.

## Problem

The test coverage for adapters in `crates/mev-internal/src/adapters/` and `src/adapters/` has critical gaps that compromise error handling and correct adapter integration. Several adapters are missing testing entirely.

## Evidence

- source_event: "adapters_coverage_gap_cov.md"
  path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "0/31 lines"
  note: "0% test coverage."
- source_event: "adapters_coverage_gap_cov.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/25 lines"
  note: "0% test coverage."
- source_event: "adapters_coverage_gap_cov.md"
  path: "crates/mev-internal/src/adapters/process.rs"
  loc: "0/11 lines"
  note: "0% test coverage."

## Change Scope

- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/process.rs`

## Constraints

- Test external integration boundaries effectively.
- Mocking responses in an integration setting or ensuring interactions conform to expected specs is required.

## Acceptance Criteria

- Code coverage for `crates/mev-internal/src/adapters/gh.rs` is at an acceptable level.
- Code coverage for `crates/mev-internal/src/adapters/git.rs` is at an acceptable level.
- Code coverage for `crates/mev-internal/src/adapters/process.rs` is at an acceptable level.
