---
label: "tests"
created_at: "2024-03-23"
author_role: "cov"
confidence: "high"
---

## Problem

The VCS identity switching orchestration logic in `src/app/commands/switch/mod.rs` has 0% line coverage. This represents an unmonitored risk to a critical authentication/state transition path.

## Goal

Ensure that identity switching behavior is covered by integration tests at the application boundary to catch regressions in setting proper global Git and Jujutsu configuration states.

## Context

Coverage metrics (via `cargo tarpaulin`) indicate that the entirety of the `switch` command orchestration—including loading identities from the store, validating the identity configuration, and writing values to the VCS ports—is uncovered. As this modifies global tool configurations on the developer's system, a silent failure here could lead to commits being authored with incorrect personal or work identities.

## Evidence

- path: "src/app/commands/switch/mod.rs"
  loc: "14-43"
  note: "0/23 lines covered. The `execute` function performs a critical auth/state transition but lacks integration tests validating the success or failure paths."

## Change Scope

- `src/app/commands/switch/mod.rs`
- `tests/cli_contracts/switch.rs` (to be created or modified)