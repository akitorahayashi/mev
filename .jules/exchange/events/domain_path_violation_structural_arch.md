---
label: "refacts"
created_at: "2024-03-12"
author_role: "structural_arch"
confidence: "high"
---

## Problem

Domain pure logic ports (`FsPort` and `IdentityStore`) depend on `std::path::Path` and `std::path::PathBuf` structs, entangling the domain layer with host filesystem concepts.

## Goal

Decouple I/O concerns from the domain layer by replacing `std::path` types in domain ports with primitive types like `&str` or `String`.

## Context

The Architecture Rule dictates that domain pure logic ports must abstract file system concepts away and avoid `std::path` types. Depending on `std::path` couples the port contracts to a specific implementation detail (filesystem paths) when they should be strictly abstract string/primitive representations.

## Evidence

- path: "src/domain/ports/fs.rs"
  loc: "Lines 3, 10-20"
  note: "Imports `std::path::{Path, PathBuf}` and uses them in all port method signatures (`exists`, `read_to_string`, `read_dir`, `write`, `create_dir_all`)."
- path: "src/domain/ports/identity_store.rs"
  loc: "Lines 3, 21"
  note: "Imports `std::path::PathBuf` and uses it as the return type for the `identity_path` method."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/fs/std_fs.rs`
- `src/adapters/identity_store/local_json.rs`
- `src/app/commands/*`
