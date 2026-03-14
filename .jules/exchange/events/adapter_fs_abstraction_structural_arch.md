---
label: "refacts"
created_at: "2026-03-14"
author_role: "structural_arch"
confidence: "medium"
---

## Problem

Some files interact with `std::fs` explicitly in the app/command layer, rather than utilizing the `FsPort`. This bypasses testing seams and couples orchestration logic directly to the filesystem adapter.

## Goal

Consistent abstraction over I/O. All application and command orchestration logic must use `ctx.fs` (`FsPort`) rather than relying on `std::fs`, ensuring proper decoupling and testability via dependency injection.

## Context

While `FsPort` exists and is wired up in `DependencyContainer`, there are instances where `std::fs` is used directly in commands, which creates a testing blast radius and bypasses the defined boundary.

## Evidence

- path: "src/app/commands/deploy_configs.rs"
  loc: "42"
  note: "Uses `std::fs::remove_dir_all(&target)` instead of abstracting through the `FsPort`."

- path: "src/app/commands/deploy_configs.rs"
  loc: "66-74"
  note: "`copy_dir_recursive` uses `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy` instead of port abstractions."

- path: "src/app/commands/config/mod.rs"
  loc: "63-71"
  note: "`copy_dir_recursive` uses `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy` instead of port abstractions."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/domain/ports/fs.rs`