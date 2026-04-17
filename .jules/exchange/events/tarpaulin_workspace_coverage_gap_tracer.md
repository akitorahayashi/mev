---
label: "tests"
created_at: "2026-04-17"
author_role: "tracer"
confidence: "high"
---

## Problem

The test coverage report artificially reports 0% line coverage for the `mev-internal` crate and its internal dependencies (e.g. `crates/mev-internal/src/domain/repository_ref.rs: 0/50`) because the `just coverage` task uses `cargo tarpaulin` without executing tests within internal crates. This hides actual regressions and presents false safety risks, rendering the global coverage target (fail-under 40) unreliable and misleading.

## Goal

Correct the Tarpaulin execution boundary to include sub-crate test execution, ensuring coverage metrics accurately reflect all workspace crates' tested behavior, enabling meaningful line and condition regression detection.

## Context

Tarpaulin is currently configured via `just coverage` using `cargo tarpaulin --packages mev` or equivalent, which explicitly executes only the root crate's tests. Because `mev-internal` tests run independently due to dev-dependencies, its coverage results are completely missed during `just coverage`, suppressing critical signal on core domain logic. Coverage should be evaluated across the whole workspace (or internal crates individually) to serve as a reliable risk metric.

## Evidence

- path: "justfile"
  loc: "coverage: ... cargo tarpaulin"
  note: "Tarpaulin task executes without specifying workspace or testing internal crates, failing to capture their coverage."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/50"
  note: "Critical domain logic parsing repository references shows 0% coverage despite having existing tests, because Tarpaulin ignores the sub-crate execution."

## Change Scope

- `justfile`
