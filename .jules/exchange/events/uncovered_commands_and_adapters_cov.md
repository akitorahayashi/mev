---
label: "tests"
created_at: "2024-05-24"
author_role: "cov"
confidence: "high"
---

## Problem

Several critical CLI commands, adapters, and domain modules completely lack test coverage, specifically in `crates/mev-internal`. The global line coverage metric is artificially boosted while these essential paths remain untested, creating a high risk of regression.

## Goal

Establish foundational test coverage for the untrusted/uncovered boundary interfaces and CLI logic to ensure failure-expensive decisions are verified. Focus first on writing tests for the `gh` and `git` internal CLI tools and their related domain logic.

## Context

The `cargo tarpaulin` output indicates 0% coverage across the following key areas:
- `crates/mev-internal/src/app/commands/gh/`
- `crates/mev-internal/src/app/commands/git/`
- `crates/mev-internal/src/app/cli/`
- `crates/mev-internal/src/adapters/`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`

Without test boundaries on these modules, any underlying adapter changes could silently break the `gh` and `git` integration.

## Evidence

- path: "crates/mev-internal/src/app/cli/gh.rs"
  loc: "0/7"
  note: "Only tests exist for subcommand shape but zero line coverage recorded in tarpaulin output."
- path: "crates/mev-internal/src/app/cli/git.rs"
  loc: "0/3"
  note: "No coverage for CLI execution flow."
- path: "crates/mev-internal/src/app/commands/gh/labels_deploy.rs"
  loc: "0/15"
  note: "Core command logic completely uncovered."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "0/37"
  note: "System boundary for git operations is completely untested."

## Change Scope

- `crates/mev-internal/tests/`
- `crates/mev-internal/src/app/commands/`
- `crates/mev-internal/src/adapters/`
