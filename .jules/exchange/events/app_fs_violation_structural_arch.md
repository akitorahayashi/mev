---
label: "refacts"
created_at: "2024-03-12"
author_role: "structural_arch"
confidence: "high"
---

## Problem

The `app/commands` layer (specifically `deploy_configs.rs` and `config/mod.rs`) directly imports and uses `std::fs` and `std::path` to perform filesystem mutations, rather than delegating I/O operations through the `FsPort`.

## Goal

Enforce the dependency boundary by routing all filesystem interactions in the application logic layer through the `FsPort`, abstracting away direct host system dependencies and improving testability.

## Context

The application orchestration commands should not bypass the port/adapter boundary to perform I/O side effects directly. By using standard library filesystem routines, the code tightly couples to the host filesystem, preventing simple dependency injection for testing and breaking the designed abstraction.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "Lines 42-73"
  note: "Directly calls `std::fs::remove_dir_all`, `std::fs::rename`, `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."
- path: "src/app/commands/deploy_configs.rs"
  loc: "Lines 41-76"
  note: "Directly calls `std::fs::remove_dir_all`, `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy`."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/domain/ports/fs.rs`
- `src/adapters/fs/std_fs.rs`
