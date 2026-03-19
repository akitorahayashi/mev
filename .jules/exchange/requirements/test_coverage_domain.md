---
label: "tests"
implementation_ready: false
---

## Goal

Add comprehensive test coverage to the domain module in `crates/mev-internal` to ensure critical paths, data structures, and constraints are explicitly validated and regression gates are strengthened.

## Problem

The test coverage for domain models in `crates/mev-internal/src/domain/` is severely lacking, with key structs entirely uncovered. Leaving critical paths such as domain decisions and data validation uncovered increases the likelihood of unnoticed regressions. Several modules like `repository_ref`, `submodule_path`, and `repo_target` have 0% coverage. These structures likely model the core domain logic for git repositories and configurations, making their coverage crucial for maintaining code quality.

## Evidence

- source_event: "mev_internal_domain_coverage_gap_cov.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/48 lines"
  note: "0% test coverage."
- source_event: "mev_internal_domain_coverage_gap_cov.md"
  path: "crates/mev-internal/src/domain/submodule_path.rs"
  loc: "0/11 lines"
  note: "0% test coverage."
- source_event: "mev_internal_domain_coverage_gap_cov.md"
  path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "0/7 lines"
  note: "0% test coverage."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/repo_target.rs`

## Constraints

- Test external integration boundaries effectively.

## Acceptance Criteria

- Code coverage for `crates/mev-internal/src/domain/repository_ref.rs` is at an acceptable level.
- Code coverage for `crates/mev-internal/src/domain/submodule_path.rs` is at an acceptable level.
- Code coverage for `crates/mev-internal/src/domain/repo_target.rs` is at an acceptable level.
