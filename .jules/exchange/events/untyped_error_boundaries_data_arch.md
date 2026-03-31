---
label: "refacts"
created_at: "2024-05-23"
author_role: "data_arch"
confidence: "high"
---

## Problem

Boundary error definitions use untyped `Box<dyn std::error::Error>` across internal crate domain logic instead of using explicit typed errors.

## Goal

Replace the generic `Box<dyn std::error::Error>` boundaries with an explicit domain typed error enum (e.g., `DomainError` or `InternalError`) that models explicit failure states.

## Context

The Boundary Sovereignty and Error Modeling principles state that boundary entry points should use explicit error types to encode expected failure states and prevent panics, ensuring callers handle errors safely. Using `Box<dyn std::error::Error>` causes caller ambiguity and masks specific operational failures.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "12-16"
  note: "`from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`."
- path: "crates/mev-internal/src/domain/submodule_path.rs"
  loc: "5"
  note: "`validate_submodule_path` returns `Result<(), Box<dyn std::error::Error>>`."
- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "6"
  note: "`resolve_repo_ref` returns `Result<RepositoryRef, Box<dyn std::error::Error>>`."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
