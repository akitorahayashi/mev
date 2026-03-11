---
label: "tests"
created_at: "2026-03-11"
author_role: "cov"
confidence: "high"
---

## Problem

Code coverage is at 20.00%, which is below the 40.00% failure threshold configured in the `just coverage` target.

## Goal

Increase test coverage across critical domain and adapter paths to exceed the 40% threshold, ensuring regression risks are mitigated and safety gates pass in CI/CD.

## Context

The `coverage` recipe in `justfile` enforces a 40% code coverage threshold utilizing `cargo-tarpaulin`. The current run resulted in only `20.00% coverage, 238/1190 lines covered`, failing the threshold. Critical domains like `crates/mev-internal/src/domain/repo_target.rs`, `crates/mev-internal/src/domain/repository_ref.rs`, and adapter components (`gh`, `git`, `process`) have `0/XX` lines covered. This signals significant gaps in verifying core system logic.

## Evidence

- path: "justfile"
  loc: "line 77"
  note: "Specifies --fail-under 40 for cargo tarpaulin."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "Entire file"
  note: "Coverage Results indicate 0/48 lines tested."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "Entire file"
  note: "Coverage Results indicate 0/25 lines tested."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
