---
label: "tests"
created_at: "2025-05-24"
author_role: "tracer"
confidence: "high"
---

## Problem

The internal crate modules `crates/mev-internal/src/domain/repo_target.rs` and `crates/mev-internal/src/domain/repository_ref.rs` are reported as having 0% code coverage despite having comprehensive unit test modules in their files.

## Goal

Determine why the `cargo tarpaulin` coverage report generated via `just coverage` is completely omitting the test execution for `mev-internal` unit tests, and adjust the coverage command or testing bounds to capture these critical path assertions.

## Context

The `repo_target` and `repository_ref` modules validate and resolve Git remote targets. Failure in this path will break label provisioning tools. The codebase clearly contains tests for these domains (e.g. `prefers_explicit_repo`, `parses_owner_name_repo_arg`), but they are not reflected in the coverage metrics. This is a false negative in the risk signal because `just coverage` calls `cargo tarpaulin --packages mev`, explicitly omitting `mev-internal` from the report.

## Evidence

- path: "justfile"
  loc: "line 53 (`--packages mev`)"
  note: "The tarpaulin invocation is restricted to the `mev` package and explicitly excludes workspace members."
- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "0/8 lines covered"
  note: "Tests are present in the file but not run during the coverage step."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/50 lines covered"
  note: "Tests are present in the file but not run during the coverage step."

## Change Scope

- `justfile`
