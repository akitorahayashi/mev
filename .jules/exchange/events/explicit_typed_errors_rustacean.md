---
label: "refacts"
created_at: "2024-05-15"
author_role: "rustacean"
confidence: "high"
---

## Problem

The codebase frequently uses `Box<dyn std::error::Error>` across multiple layers, including application bounds, domain parsing, and adapter functions. This erases the semantic meaning of errors, making classification impossible and diagnosis less actionable, directly violating the architectural rule: "All domain and boundary errors must use explicit typed errors (e.g., `DomainError` in internal crates or `AppError` at the application layer) instead of generic `Box<dyn std::error::Error>`."

## Goal

Replace all instances of `Box<dyn std::error::Error>` with explicitly typed errors (e.g., `DomainError`, `AppError`, `AdapterError` or standard enum errors per boundary) to preserve error context and semantic classification.

## Context

Using `Box<dyn std::error::Error>` collapses all errors into a generic type, making it impossible for upstream callers to distinguish between different failure modes (e.g., IO vs network vs parsing). By using explicitly typed errors, the codebase adheres to the standard error handling practices where errors are part of the contract and context is attached.

## Evidence

- path: "src/app/container.rs"
  loc: "40, 60"
  note: "Uses Box<dyn std::error::Error> as the return type for new() and for_identity()."
- path: "crates/mev-internal/src/app/commands/git/delete_submodule.rs"
  loc: "14"
  note: "Uses Box<dyn std::error::Error> as the return type for run()."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "11, 20, 51, 64, 71, 78, 85"
  note: "Parsing functions return Box<dyn std::error::Error>."
- path: "crates/mev-internal/src/app/cli/gh.rs"
  loc: "12, 27"
  note: "Uses Box<dyn std::error::Error>."
- path: "crates/mev-internal/src/app/cli/git.rs"
  loc: "11"
  note: "Uses Box<dyn std::error::Error>."
- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "8, 20"
  note: "Uses Box<dyn std::error::Error>."

## Change Scope

- `src/app/container.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/app/cli/gh.rs`
- `crates/mev-internal/src/app/cli/git.rs`
- `crates/mev-internal/src/adapters/process.rs`
