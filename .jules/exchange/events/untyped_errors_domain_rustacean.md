---
label: "refacts"
created_at: "2024-05-24"
author_role: "rustacean"
confidence: "high"
---

## Problem

Widespread use of `Box<dyn std::error::Error>` in domain logic (e.g., `RepositoryRef`) and application boundaries, losing typed error semantics and classification.

## Goal

Replace dynamic error trait objects with typed domain errors (like `AppError` or a specialized enum) to preserve domain meaning and context after propagation.

## Context

Using `Box<dyn Error>` collapses typed errors, preventing downstream callers from meaningfully matching on failure variants or adding context based on the operation. This violates the "Errors are part of the contract" principle.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "from_repo_arg, from_remote_url"
  note: "`from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`, erasing the distinction between parsing and IO failures."

- path: "src/app/container.rs"
  loc: "new, for_identity"
  note: "`Container::new` and `Container::for_identity` return dynamic errors instead of `AppError`."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
- `src/app/container.rs`