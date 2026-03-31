---
label: "refacts"
implementation_ready: false
---

## Goal

Replace the generic `Box<dyn std::error::Error>` boundaries with an explicit domain typed error enum (e.g., `DomainError` or `InternalError`) that models explicit failure states.

## Problem

The internal crate (`crates/mev-internal`) and some adapters use `Box<dyn std::error::Error>` extensively for error handling, which violates the architecture rule specifying that typed errors must be used instead.

## Context

Using `Box<dyn std::error::Error>` collapses errors, making it difficult to match on or handle specific error types at the boundary. The architecture rule for Error Types requires the use of explicit typed errors across internal crates to ensure errors are part of the contract and context is maintained.
The Boundary Sovereignty and Error Modeling principles state that boundary entry points should use explicit error types to encode expected failure states and prevent panics, ensuring callers handle errors safely. Using `Box<dyn std::error::Error>` causes caller ambiguity and masks specific operational failures.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "line 12, 20"
  note: "`from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`"
- path: "crates/mev-internal/src/domain/submodule_path.rs"
  loc: "line 5"
  note: "`validate_submodule_path` returns `Result<(), Box<dyn std::error::Error>>`"
- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "line 5, 15"
  note: "`run_status` and `run_output` return `Result<_, Box<dyn std::error::Error>>`"
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "12-16"
  note: "`from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`."
- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "6"
  note: "`resolve_repo_ref` returns `Result<RepositoryRef, Box<dyn std::error::Error>>`."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`

## Constraints

- Preserve existing error context when mapping to typed errors.

## Acceptance Criteria

- Typed errors are used in `mev-internal` and boundaries instead of generic `Box<dyn std::error::Error>`.
