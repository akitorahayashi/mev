---
label: "refacts"
created_at: "2026-03-11"
author_role: "data_arch"
confidence: "high"
---

## Problem

The function `copy_dir_recursive` is duplicated across multiple modules (`src/app/commands/config/mod.rs` and `src/app/commands/deploy_configs.rs`), bypassing the `FsPort` abstraction designed to decouple I/O concerns from domain and application layers.

## Goal

Consolidate the recursive directory copying logic by moving it into the `FsPort` abstraction (and implementing it in `StdFs`). This enforces a single source of truth for filesystem operations and eliminates redundant code.

## Context

The repository has defined an `FsPort` to abstract file operations, ensuring I/O concerns do not leak into application logic. However, `copy_dir_recursive` currently lives as raw `std::fs` calls in the application layer (`src/app/commands/config/mod.rs` and `src/app/commands/deploy_configs.rs`). This violates both the Single Source of Truth principle (for I/O operations) and Boundary Sovereignty (by using raw `std::path` operations directly in commands without going through the defined port).

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "62-74"
  note: "Implements `copy_dir_recursive` directly."
- path: "src/app/commands/deploy_configs.rs"
  loc: "58-75"
  note: "Implements a nearly identical `copy_dir_recursive` directly."
- path: "src/domain/ports/fs.rs"
  loc: "7-21"
  note: "`FsPort` is the canonical owner of filesystem logic but is missing directory copy capabilities."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/adapters/fs/std_fs.rs`
- `src/domain/ports/fs.rs`