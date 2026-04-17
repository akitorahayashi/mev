---
label: "refacts"
created_at: "2024-04-17"
author_role: "factorer"
confidence: "high"
---

## Problem

Wrapper Sprawl and Misplaced Code: The `copy_dir_recursive` function is defined in `src/app/commands/deploy_configs.rs`, but it is also used by `src/app/commands/config/mod.rs`. This function is a pure filesystem operation wrapper (`FsPort`) and does not contain any application/domain logic specific to deploying configurations.

## Goal

Relocate `copy_dir_recursive` to act as a shared utility. As an implementation acting over `FsPort`, it conceptually belongs to the adapter layer where file system operations are implemented (e.g. `src/adapters/fs.rs`), or as a default implementation on the `FsPort` trait itself.

## Context

Functions should live where their authority and conceptual ownership lie. A generic filesystem utility should not be coupled with the specific `deploy_configs` command logic.

## Evidence

- path: "src/app/commands/deploy_configs.rs"
  loc: "52-69"
  note: "Definition of copy_dir_recursive"
- path: "src/app/commands/config/mod.rs"
  loc: "55"
  note: "External caller of copy_dir_recursive"

## Change Scope

- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`
- `src/domain/ports/fs.rs`
- `src/adapters/fs.rs`
