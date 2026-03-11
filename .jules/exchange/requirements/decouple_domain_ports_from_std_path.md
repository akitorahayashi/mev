---
label: "refacts"
implementation_ready: false
---

## Goal

Abstract file system concepts away by replacing `std::path::Path` and `std::path::PathBuf` types in domain logic ports with primitive types like `&str` or `String`.

## Problem

Domain ports are directly using `std::path::Path` and `std::path::PathBuf` types. This introduces I/O coupling into pure logic boundaries, violating the architecture rule that domain pure logic ports must abstract file system concepts away. It couples internal domain logic to host environment file primitives.

## Evidence

- source_event: "domain_ports_std_path_structural_arch.md"
  path: "src/domain/ports/fs.rs"
  loc: "5"
  note: "`use std::path::{Path, PathBuf};` directly relies on `std::path` types for the `FsPort` trait."
- source_event: "domain_ports_std_path_structural_arch.md"
  path: "src/domain/ports/identity_store.rs"
  loc: "5"
  note: "`use std::path::PathBuf;` directly relies on `std::path` types for the `IdentityStore` trait."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`

## Constraints

- Domain pure logic ports must abstract file system concepts away and avoid `std::path` types (using primitive types like `&str` or `String`) to decouple I/O concerns.

## Acceptance Criteria

- `src/domain/ports/fs.rs` defines methods accepting and returning strings or standard primitive slices instead of `Path` or `PathBuf`.
- `src/domain/ports/identity_store.rs` uses string primitives instead of path buffers.
