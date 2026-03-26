---
label: "refacts"
implementation_ready: false
---

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`, `Cow`) where appropriate.

## Problem

The `.clone()` usage is prolific throughout the codebase, often used reflexively to appease the borrow checker rather than to express a deliberate design intent. Several instances copy strings, vectors, and path buffers when borrowed values (`&str`, `&Path`, `Cow`) would suffice, leading to unnecessary allocations.

## Evidence

- source_event: "unnecessary_clones_rustacean.md"
  path: "src/domain/execution_plan.rs"
  loc: "43"
  note: "Clones the `tags` parameter instead of passing it explicitly or allowing an owned variant."

## Change Scope

- `src/domain/execution_plan.rs`
- `src/adapters/ansible/locator.rs`
- `src/adapters/ansible/executor.rs`

## Constraints

- Refactor code to borrow values instead of cloning where references are sufficient.

## Acceptance Criteria

- Reflexive `.clone()` usages in the specified files are removed and replaced with safe borrows.