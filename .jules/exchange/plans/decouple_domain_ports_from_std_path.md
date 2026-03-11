---
label: "refacts"
---

## Goal

Abstract file system concepts away by replacing `std::path::Path` and `std::path::PathBuf` types in domain logic ports with primitive types like `&str` or `String`.

## Problem

Domain ports are directly using `std::path::Path` and `std::path::PathBuf` types. This introduces I/O coupling into pure logic boundaries, violating the architecture rule that domain pure logic ports must abstract file system concepts away. It couples internal domain logic to host environment file primitives.

## Affected Areas

### Domain Ports

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`

## Constraints

- Domain pure logic ports must abstract file system concepts away and avoid `std::path` types (using primitive types like `&str` or `String`) to decouple I/O concerns.

## Risks

- Implementations of `FsPort` and `IdentityStore` (adapters) will need to be updated to match the new trait signatures, which will require internal conversion from `&str` to `Path` and `PathBuf` to `String`.
- Domain services and application logic calling these ports will fail to compile until they are updated to use string primitives instead of path types.

## Acceptance Criteria

- `src/domain/ports/fs.rs` defines methods accepting and returning strings or standard primitive slices instead of `Path` or `PathBuf`.
- `src/domain/ports/identity_store.rs` uses string primitives instead of path buffers.
- The project successfully compiles and all tests pass with the updated port boundaries.

## Implementation Plan

1. Update `src/domain/ports/fs.rs` to remove `use std::path::{Path, PathBuf};` and replace all occurrences of `&Path` with `&str` and `PathBuf` with `String`.
2. Update `src/domain/ports/identity_store.rs` to remove `use std::path::PathBuf;` and replace `PathBuf` with `String` in the `identity_path` method return type.
3. Use `rg FsPort` and `rg IdentityStore` to find adapter implementations of these traits and update their signatures to match the new trait definitions. Adapter implementations should handle conversions internally (e.g., `Path::new(path)` for filesystem operations, or `.to_string_lossy().to_string()` for returning paths as strings).
4. Run `cargo check` to identify callers in the domain and application layers that fail to compile due to passing `&Path` or receiving `PathBuf`.
5. Update all identified callers to pass `&str` and expect `String` instead of using `std::path` primitives.
6. Run `just test` to verify that all tests pass and external behavior remains unaffected.
7. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
