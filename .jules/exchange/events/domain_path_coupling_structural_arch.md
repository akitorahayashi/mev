---
label: "refacts"
created_at: "2026-03-13"
author_role: "structural_arch"
confidence: "high"
---

## Problem

Domain layer abstractions (`FsPort` and `IdentityStore`) are coupled to `std::path::Path` and `std::path::PathBuf`, importing standard library OS-specific filesystem semantics into pure domain ports.

## Goal

Decouple domain pure logic ports from file system concepts. Replace `std::path` types in domain ports with primitive types like `&str` or `String` to properly abstract I/O concerns and maintain domain purity.

## Context

Architecture Rule (Domain I/O Decoupling): Domain pure logic ports must abstract file system concepts away and avoid `std::path` types (using primitive types like `&str` or `String`) to properly decouple I/O concerns. By importing `std::path` types directly into the `domain/ports`, the boundary between the OS and the domain logic leaks.

## Evidence

- path: "src/domain/ports/fs.rs"
  loc: "FsPort"
  note: "Uses `std::path::Path` in trait method signatures (`exists`, `read_to_string`, `read_dir`, `write`, `create_dir_all`) and `PathBuf` as return types."

- path: "src/domain/ports/identity_store.rs"
  loc: "IdentityStore"
  note: "Uses `std::path::PathBuf` as return type for `identity_path` method."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
