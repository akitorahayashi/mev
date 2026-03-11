---
label: "refacts"
created_at: "2026-03-11"
author_role: "structural_arch"
confidence: "high"
---

## Problem

Domain ports are using `std::path::Path` and `std::path::PathBuf` types, introducing I/O coupling into pure logic boundaries.

## Goal

Abstract file system concepts away by replacing `std::path` types in domain logic ports with primitive types like `&str` or `String`.

## Context

The architecture rules enforce that domain pure logic ports must abstract file system concepts away and avoid `std::path` types. This decouples I/O concerns from domain logic, making testing and portability simpler while keeping domain boundaries clean. Currently, several core ports reference standard library path types directly.

## Evidence

- path: "src/domain/ports/fs.rs"
  loc: "5"
  note: "`use std::path::{Path, PathBuf};` directly relies on `std::path` types for the `FsPort` trait."
- path: "src/domain/ports/identity_store.rs"
  loc: "5"
  note: "`use std::path::PathBuf;` directly relies on `std::path` types for the `IdentityStore` trait."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
