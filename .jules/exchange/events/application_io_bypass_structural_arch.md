---
label: "refacts"
created_at: "2026-03-13"
author_role: "structural_arch"
confidence: "high"
---

## Problem

The application orchestration commands (`config` and `deploy_configs`) bypass the `FsPort` abstraction and perform I/O side effects directly using `std::fs`.

## Goal

Enforce the boundary between application logic and I/O side effects. Application commands should use the `FsPort` abstraction to manipulate the filesystem, allowing dependency injection and test isolation.

## Context

Application orchestration commands must not bypass the port/adapter boundary to perform I/O side effects directly. All file system manipulations in the application layer must be delegated to the abstract `FsPort` rather than importing `std::fs` or `std::path` directly. Directly using `std::fs` violates the Architectural Rule (Application I/O Side Effects) by preventing the orchestration layer from being unit-tested without side effects.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "create"
  note: "Directly calls `std::fs::remove_dir_all`, `std::fs::rename`."

- path: "src/app/commands/config/mod.rs"
  loc: "copy_dir_recursive"
  note: "Directly uses `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."

- path: "src/app/commands/deploy_configs.rs"
  loc: "deploy_for_tags"
  note: "Directly calls `std::fs::remove_dir_all`."

- path: "src/app/commands/deploy_configs.rs"
  loc: "copy_dir_recursive"
  note: "Directly uses `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
