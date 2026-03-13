---
label: "refacts"
created_at: "2024-05-16"
author_role: "rustacean"
confidence: "high"
---

## Problem

Domain pure logic ports (`FsPort` and `IdentityStore`) leak file system concepts by exposing `std::path::{Path, PathBuf}` directly in their trait signatures.

## Goal

Abstract file system concepts away and avoid `std::path` types in domain ports. Primitive types like `&str` or `String` should be used instead to properly decouple I/O concerns from domain concepts.

## Context

According to the Architecture Rule (Domain I/O Decoupling), domain pure logic ports must abstract file system concepts away and avoid `std::path` types to properly decouple I/O concerns. Using `Path` or `PathBuf` binds the domain model directly to the host operating system's filesystem abstractions.

## Evidence

- path: "src/domain/ports/fs.rs"
  loc: "use std::path::{Path, PathBuf};"
  note: "FsPort leaks std::path::Path and std::path::PathBuf in trait method signatures."

- path: "src/domain/ports/identity_store.rs"
  loc: "use std::path::PathBuf;"
  note: "IdentityStore leaks std::path::PathBuf in trait method signatures."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
