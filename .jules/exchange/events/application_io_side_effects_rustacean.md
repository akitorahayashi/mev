---
label: "refacts"
created_at: "2024-05-16"
author_role: "rustacean"
confidence: "high"
---

## Problem

Application orchestration commands bypass the port/adapter boundary and perform I/O side effects directly. `src/app/commands/deploy_configs.rs` manipulates the file system via `std::fs` and `src/app/commands/backup/mod.rs` accesses the `HOME` environment variable via `std::env::var`.

## Goal

All file system manipulations and system interactions in the application layer must be delegated to the abstract `FsPort` or other relevant ports. The application layer must not import `std::fs` or `std::path` directly or use `std::env::var`.

## Context

According to the Architecture Rule (Application I/O Side Effects), application orchestration commands must not bypass the port/adapter boundary to perform I/O side effects directly. All file system manipulations must be delegated to the abstract `FsPort`. Similarly, system environment variables should be accessed via a port or provided via dependency injection rather than accessed globally.

## Evidence

- path: "src/app/commands/deploy_configs.rs"
  loc: "use std::path::Path;"
  note: "Imports std::path directly."

- path: "src/app/commands/deploy_configs.rs"
  loc: "std::fs::remove_dir_all(&target)"
  note: "Bypasses FsPort to manipulate the filesystem."

- path: "src/app/commands/deploy_configs.rs"
  loc: "std::fs::create_dir_all(dst)?;"
  note: "Bypasses FsPort to manipulate the filesystem."

- path: "src/app/commands/deploy_configs.rs"
  loc: "std::fs::read_dir(src)?;"
  note: "Bypasses FsPort to manipulate the filesystem."

- path: "src/app/commands/deploy_configs.rs"
  loc: "std::fs::copy(&src_path, &dst_path)?;"
  note: "Bypasses FsPort to manipulate the filesystem."

- path: "src/app/commands/backup/mod.rs"
  loc: "std::env::var(\"HOME\")"
  note: "Bypasses the port boundary to access the environment directly."

## Change Scope

- `src/app/commands/deploy_configs.rs`
- `src/app/commands/backup/mod.rs`
