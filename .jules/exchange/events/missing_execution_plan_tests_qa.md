---
label: "tests"
created_at: "2026-03-14"
author_role: "qa"
confidence: "high"
---

## Problem

Pure domain logic in `ExecutionPlan` lacks isolated unit tests, missing validation of its properties and boundaries.

## Goal

Add unit tests within `src/domain/execution_plan.rs` to verify plan construction, ensuring deterministic behavior independent of external side effects.

## Context

Domain models must enforce invariants at the boundary and be thoroughly tested for expected properties. The absence of tests for `ExecutionPlan` leaves a gap in verifying whether `full_setup` correctly includes `FULL_SETUP_TAGS` or `make` accurately handles user tags.

## Evidence

- path: "src/domain/execution_plan.rs"
  loc: "12-22"
  note: "`ExecutionPlan::full_setup` and `ExecutionPlan::make` contain pure plan construction logic but have no corresponding `#[cfg(test)]` module, unlike other domain modules (e.g., `profile.rs`, `tag.rs`)."

## Change Scope

- `src/domain/execution_plan.rs`
