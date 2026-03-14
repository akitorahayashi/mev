---
label: "refacts"
created_at: "2026-03-14"
author_role: "taxonomy"
confidence: "high"
---

## Problem
The domain model uses `Profile::Common` as a variant alongside machine-specific profiles (`Macbook`, `MacMini`), confusing a foundational configuration layer with an actual target machine profile.

## Goal
Establish a distinct domain concept (e.g., `BaseLayer` or `WorkspaceConfig`) to replace `Common` so that target profiles and foundational configurations are not conflated in the same enum.

## Context
"Common" is an ambiguous term that violates repository naming conventions. By placing `Common` inside the `Profile` enum, it implies a user could "provision a common machine," which is semantically incorrect. This also forces awkward logic like `profile.is_machine_profile()` to separate real targets from shared layers.

## Evidence
- path: "src/domain/profile.rs"
  loc: "Profile::Common"
  note: "`Common` is defined as a `Profile` variant alongside `Macbook` and `MacMini`."
- path: "src/domain/profile.rs"
  loc: "validate_machine_profile_rejects_common"
  note: "`validate_machine_profile_rejects_common()` test explicitly acknowledges `common` is not a real machine profile."

## Change Scope
- `src/domain/profile.rs`
- `src/app/commands/list/mod.rs`
- `src/app/commands/make/mod.rs`
