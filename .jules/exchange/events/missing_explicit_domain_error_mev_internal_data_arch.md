---
label: "refacts"
created_at: "2024-03-26"
author_role: "data_arch"
confidence: "high"
---

## Problem

The internal crate `mev-internal` uses `Box<dyn std::error::Error>` across its domain models, application logic, and adapter boundaries instead of an explicit typed error.

## Goal

Introduce an explicit `DomainError` in `mev-internal` to represent boundary and domain-level failures and replace instances of `Box<dyn std::error::Error>` to enforce error-handling invariants.

## Context

According to the Architecture Rule (Error Types), "All domain and boundary errors must use explicit typed errors (e.g., DomainError in internal crates or AppError at the application layer) instead of generic Box<dyn std::error::Error>." The generic error box obscures error details and causes implicit validation handling at call sites instead of explicitly matching domain errors.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "from_repo_arg()"
  note: "`from_repo_arg` returns `Result<Self, Box<dyn std::error::Error>>` instead of a typed error."

- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "resolve_repo_ref()"
  note: "`resolve_repo_ref` returns `Result<RepositoryRef, Box<dyn std::error::Error>>`."

- path: "crates/mev-internal/src/domain/submodule_path.rs"
  loc: "validate_submodule_path()"
  note: "`validate_submodule_path` returns `Result<(), Box<dyn std::error::Error>>`."

- path: "crates/mev-internal/src/domain/label_catalog.rs"
  loc: "load_bundled_labels()"
  note: "`load_bundled_labels` returns `Result<Vec<LabelSpec>, Box<dyn std::error::Error>>`."

## Change Scope

- `crates/mev-internal/src/domain/error.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/process.rs`
