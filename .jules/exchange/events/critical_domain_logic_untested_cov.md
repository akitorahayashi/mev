---
label: "tests"
created_at: "2026-03-11"
author_role: "cov"
confidence: "high"
---

## Problem

Critical domain transitions and decision paths in `repo_target.rs` and `repository_ref.rs` have zero line coverage, creating high regression risk in core routing and identification logic.

## Goal

Ensure key domain entities are unit tested for valid state transitions and boundaries, reducing reliance on macro-level or manual testing for critical path correctness.

## Context

Coverage metrics highlight `crates/mev-internal/src/domain/repo_target.rs` (0/7 lines) and `crates/mev-internal/src/domain/repository_ref.rs` (0/48 lines) are completely lacking unit tests. As domain models, they should be isolated from I/O and heavily tested with unit tests (`#[cfg(test)]`) per testing strategy constraints to validate internal logic handling correctly.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "Entire file"
  note: "0/48 lines covered according to tarpaulin report. This indicates untested logic paths."
- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "Entire file"
  note: "0/7 lines covered. Domain entities are fundamentally untested."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
