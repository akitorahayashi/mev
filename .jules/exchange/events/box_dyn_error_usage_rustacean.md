---
label: "refacts"
created_at: "2024-03-31"
author_role: "rustacean"
confidence: "high"
---

## Problem

The internal crate (`crates/mev-internal`) and some adapters use `Box<dyn std::error::Error>` extensively for error handling, which violates the architecture rule specifying that typed errors (e.g. `DomainError` or `AppError`) must be used instead of generic `Box<dyn std::error::Error>`.

## Goal

Refactor error handling in `crates/mev-internal` to use a typed domain error (e.g. `InternalError` or `DomainError`) that preserves error semantics and classification instead of falling back to untyped `Box<dyn std::error::Error>`. Update function signatures in `mev-internal/src/domain/` and `mev-internal/src/adapters/` accordingly.

## Context

Using `Box<dyn std::error::Error>` collapses errors, making it difficult to match on or handle specific error types at the boundary. The architecture rule for Error Types requires the use of explicit typed errors across internal crates to ensure errors are part of the contract and context is maintained.

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

## Change Scope

- `crates/mev-internal/src/domain/mod.rs`
- `crates/mev-internal/src/domain/error.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
