---
label: "refacts"
created_at: "2026-04-17"
author_role: "modeler"
confidence: "low"
---

## Problem

Error construction in `AppError` is stringly-typed and does not enforce invariants at the boundary for which strings are valid `AppError::InvalidTag` or `AppError::InvalidProfile`.

## Goal

Encode specific failed parsed values into the error enum instead of stringly-typed messages where possible, or add more structured context, so error modeling captures the invalid state explicitly rather than building display strings during construction.

## Context

`AppError::InvalidTag(String)` stores a formatted error message rather than the invalid tag itself (e.g. `AppError::InvalidTag(String)` instead of `AppError::InvalidTag(Tag)` or `AppError::InvalidTag(RawTag)`). This limits the ability to match on or inspect the exact input that failed, binding the error structure to display logic too early.

## Evidence

- path: "src/domain/error.rs"
  loc: "12"
  note: "`InvalidProfile(String)`"
- path: "src/domain/error.rs"
  loc: "18"
  note: "`InvalidTag(String)`"
- path: "src/app/commands/make/mod.rs"
  loc: "23-25"
  note: "Formats the error message directly into the `InvalidTag` enum variant: `return Err(AppError::InvalidTag(format!(\"'{t}'. Use 'mev list' to see available tags.\")));`."

## Change Scope

- `src/domain/error.rs`
- `src/app/commands/make/mod.rs`
