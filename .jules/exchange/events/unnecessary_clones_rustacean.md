---
label: "refacts"
created_at: "2024-05-15"
author_role: "rustacean"
confidence: "medium"
---

## Problem

The `.clone()` usage is prolific throughout the codebase, often used reflexively to appease the borrow checker rather than to express a deliberate design intent. Several instances copy strings, vectors, and path buffers when borrowed values (`&str`, `&Path`, `Cow`) would suffice, leading to unnecessary allocations.

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`, `Cow`) where appropriate, reducing unnecessary allocations and keeping references tighter to their data owners.

## Context

Excessive use of `.clone()` obscures the actual ownership flow and introduces unnecessary performance overhead through allocations. Aligning the architecture to favor references or explicit ownership transfers simplifies the mental model of the system.

## Evidence

- path: "src/domain/execution_plan.rs"
  loc: "43"
  note: "Clones the `tags` parameter instead of passing it explicitly or allowing an owned variant."
- path: "src/adapters/ansible/locator.rs"
  loc: "109, 110"
  note: "Clones `manifest_dir` unnecessarily instead of passing an owned or referenced path."
- path: "src/adapters/ansible/executor.rs"
  loc: "274, 288"
  note: "Unnecessary string clones when pushing tags to maps/vectors."

## Change Scope

- `src/domain/execution_plan.rs`
- `src/adapters/ansible/locator.rs`
- `src/adapters/ansible/executor.rs`
