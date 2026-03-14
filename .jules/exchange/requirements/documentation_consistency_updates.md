---
label: "docs"
implementation_ready: false
---

## Goal
Update architecture documentation to accurately reflect current codebase structure and dependencies.

## Problem
The `docs/architecture.md` file contains outdated or missing information. It references a non-existent `context.rs` instead of `container.rs`, erroneously documents a `shell` module under `crates/mev-internal`, refers to `src/domain/config.rs` which doesn't exist, and completely misses `src/domain/backup_target.rs` and `src/domain/vcs_identity.rs`. This inconsistency creates confusion for developers navigating the repository.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "docs_architecture_container_consistency.md"
  path: "docs/architecture.md"
  loc: "19"
  note: "Documents `├── context.rs          # Dependency wiring (ports → adapters)` instead of `container.rs`."
- source_event: "docs_architecture_crates_consistency.md"
  path: "docs/architecture.md"
  loc: "35"
  note: "Documents `└── mev-internal/          # Internal command implementations (shell, vcs)` though `shell` doesn't exist."
- source_event: "docs_architecture_missing_files_consistency.md"
  path: "docs/architecture.md"
  loc: "23"
  note: "Documents `├── config.rs           # VCS identity configuration model` but missing `backup_target.rs` and `vcs_identity.rs`."

## Change Scope
- `docs/architecture.md`

## Constraints
- Architectural documentation must strictly match the current directory and file structures.

## Acceptance Criteria
- `docs/architecture.md` references `container.rs`, removes the `shell` module reference, removes `config.rs`, and includes `backup_target.rs` and `vcs_identity.rs`.
