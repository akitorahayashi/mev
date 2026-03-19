---
label: "tests"
created_at: "2026-03-19"
author_role: "cov"
confidence: "high"
---

## Problem

The test coverage for adapters in `crates/mev-internal/src/adapters/` and `src/adapters/` has critical gaps that compromise error handling and correct adapter integration. Several adapters are missing testing entirely.

## Goal

Provide test coverage for external integrations (GitHub, Git, Process) to assure resilient responses, error catching on external system failures, and proper boundary transformations.

## Context

Adapters establish the boundary with external dependencies, and their failures can lead to uncontrolled and unpredictable system behavior. For `gh.rs` and `git.rs`, mocking responses in an integration setting or ensuring their interactions conform to expected specifications is key. High line coverage doesn't necessarily mean correct integration, but 0% coverage represents significant untested points of failure that can silently regress.

## Evidence

- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "0/31 lines"
  note: "0% test coverage."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/25 lines"
  note: "0% test coverage."
- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "0/11 lines"
  note: "0% test coverage."

## Change Scope

- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/process.rs`
