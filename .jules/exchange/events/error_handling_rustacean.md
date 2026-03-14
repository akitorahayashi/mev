---
label: "refacts"
created_at: "2025-03-14"
author_role: "rustacean"
confidence: "high"
---

## Problem

Heavy usage of `Box<dyn std::error::Error>` across the `crates/mev-internal` library boundary collapses typed errors, preventing consumers from correctly classifying failures or reacting to domain-specific conditions (like a missing Git section vs an I/O error).

## Goal

Define explicit error types (e.g. via `thiserror`) for internal modules (`gh`, `git`, `process`) to preserve classification and attach boundary context, removing `Box<dyn Error>` from function signatures.

## Context

The `mev-internal` crate provides CLI boundaries for `git` and `gh`. Returning type-erased errors forces callers into string-matching or generic failure handling, violating the Rust design rule that errors must preserve domain meaning and classification.

## Evidence

- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "run_status and run_output"
  note: "Returns `Result<..., Box<dyn std::error::Error>>` for process execution failures, hiding the underlying IO or exit status error types."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "RepositoryRef::from_remote_url and parsing functions"
  note: "Fails with a string converted to `Box<dyn Error>`, dropping the distinction between parse failures and domain logic errors."

## Change Scope

- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
