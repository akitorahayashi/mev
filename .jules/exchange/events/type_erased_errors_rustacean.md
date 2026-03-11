---
label: "refacts"
created_at: "2026-03-11"
author_role: "rustacean"
confidence: "high"
---

## Problem

Widespread use of type-erased `Box<dyn std::error::Error>` across the codebase (specifically in `crates/mev-internal/src/adapters/` and `src/adapters/ansible/executor.rs`), losing semantic error classification.

## Goal

Replace type-erased errors (`Box<dyn std::error::Error>`) with strongly-typed domain errors (like `AppError`) to preserve error semantics, classifications, and enable proper error handling upstream without silent fallbacks or type-guessing.

## Context

Using `Box<dyn std::error::Error>` is considered an anti-pattern in Rust libraries or domain code because it erases the specific type of the error, making it impossible for the caller to match on and handle specific failure conditions gracefully. It effectively collapses all errors into a single, opaque failure mode. According to our first principles: "Errors are part of the contract: keep semantic meaning; attach context where the system meets the world."

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "line 208"
  note: "`fn load_catalog` returns `Result<Catalog, Box<dyn std::error::Error>>` which erases the specifics of YAML parsing errors vs file IO errors."

- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "line 8"
  note: "`resolve_repo_ref` returns `Box<dyn std::error::Error>`, preventing the caller from distinguishing between parsing errors and structural errors."

- path: "crates/mev-internal/src/adapters/process.rs"
  loc: "line 8, 20"
  note: "Process execution errors are boxed, meaning calling code cannot easily distinguish between a command not found (IoError) and a non-zero exit code."

- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "lines 9, 23, 33, 46"
  note: "Git adapter operations use boxed errors, masking underlying execution or parsing failures."

- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "lines 9, 29, 39"
  note: "GitHub adapter operations return boxed errors."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/app/container.rs`
- `crates/mev-internal/src/domain/error.rs` (needs to be created/expanded)
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/app/cli/git.rs`
- `crates/mev-internal/src/app/cli/gh.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
