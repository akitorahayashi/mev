---
label: "tests"
implementation_ready: false
---

## Goal

Write unit tests to cover critical domain segments and zero-coverage modules to meet the 40% threshold requirement.

## Problem

The overall code coverage is below the required 40% threshold (at 21.13%). Critical command logic (`app/commands/config/mod.rs`, `app/commands/create/mod.rs`, etc.), domain elements, and several adapters possess 0% coverage, creating substantial risk of silent regressions.

## Evidence

- source_event: "coverage_below_threshold_cov.md"
  path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "0/31"
  note: "0% covered"
- source_event: "coverage_below_threshold_cov.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/25"
  note: "0% covered"
- source_event: "coverage_below_threshold_cov.md"
  path: "src/adapters/fs/std_fs.rs"
  loc: "0/12"
  note: "0% covered"
- source_event: "coverage_critical_paths_untested_cov.md"
  path: "src/app/commands/config/mod.rs"
  loc: "0/40"
  note: "0% test coverage for configuration settings command"
- source_event: "coverage_critical_paths_untested_cov.md"
  path: "src/app/commands/create/mod.rs"
  loc: "0/36"
  note: "0% coverage for creating environments"
- source_event: "coverage_critical_paths_untested_cov.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/48"
  note: "domain model defining critical references has 0% coverage"

## Change Scope

- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `src/adapters/fs/std_fs.rs`
- `src/app/commands/config/mod.rs`
- `src/app/commands/create/mod.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`

## Constraints

- Unit tests should mock internal adapters and focus on pure logic.

## Acceptance Criteria

- `just coverage` executes successfully and reports code coverage equal to or above 40%.
- The identified zero-coverage paths are adequately tested.
