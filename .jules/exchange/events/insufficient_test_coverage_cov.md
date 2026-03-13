---
label: "tests"
created_at: "2026-03-13"
author_role: "cov"
confidence: "high"
---

## Problem

Code coverage is below the target threshold of 40%. Several key modules have 0% line coverage, indicating a significant risk for regression.

## Goal

Increase test coverage by adding tests for the uncovered core modules to detect regressions and meet the minimum threshold.

## Context

The overall test coverage is below the 40% threshold mandated by the project. The lack of coverage in multiple modules indicates that critical paths are not validated by the test suite, allowing potential logic bugs to go unnoticed.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/48 lines covered"
  note: "0% coverage in repository ref domain."
- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "0/31 lines covered"
  note: "0% coverage in GitHub adapter."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/25 lines covered"
  note: "0% coverage in Git adapter."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`