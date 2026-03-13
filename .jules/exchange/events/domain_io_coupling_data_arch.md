---
label: "refacts"
created_at: "2026-03-13"
author_role: "data_arch"
confidence: "high"
---

## Problem

Domain pure logic ports are coupled to `std::path::PathBuf`.

## Goal

Decouple the `IdentityStore` port from standard library file system paths, using stringly or domain-specific identifiers instead if needed.

## Context

Domain pure logic ports must abstract file system concepts away. The `IdentityStore` trait currently exposes `identity_path() -> PathBuf`, forcing the domain layer to understand that identities are stored on a file system using paths.

## Evidence

- path: "src/domain/ports/identity_store.rs"
  loc: "IdentityStore::identity_path"
  note: "Returns a `std::path::PathBuf`, coupling the port to the file system."

## Change Scope

- `src/domain/ports/identity_store.rs`
- `src/adapters/identity_store/local_json.rs`
