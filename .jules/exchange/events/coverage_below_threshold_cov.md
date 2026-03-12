---
label: "tests"
created_at: "2024-03-12"
author_role: "cov"
confidence: "high"
---

## Problem

Test coverage is dangerously low at 21.13% vs a target of 40%. Several key adapters and domain modules have 0% coverage, signaling a lack of tests.

## Goal

Identify the areas driving the coverage drops and add meaningful tests to ensure critical functionality.

## Context

Running `just coverage` reveals line coverage failure:
"Coverage is below the failure threshold 21.13% < 40.00%"

## Evidence

Running `just coverage` prints the report showing missing coverage for numerous adapters and components.
- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "0/31"
  note: "0% covered"
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/25"
  note: "0% covered"
- path: "src/adapters/fs/std_fs.rs"
  loc: "0/12"
  note: "0% covered"

## Change Scope

- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `src/adapters/fs/std_fs.rs`