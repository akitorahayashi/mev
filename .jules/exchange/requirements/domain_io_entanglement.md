---
label: "refacts"
implementation_ready: false
---

## Goal

Decouple the domain interface from filesystem implementations by replacing `Path` and `PathBuf` types with domain-appropriate generic or primitive types like `&str` and `String`.

## Problem

Domain logic ports (`FsPort` and `IdentityStore`) are tightly coupled to standard library filesystem concepts (`std::path::Path` and `std::path::PathBuf`).

## Context

Per the "Architecture Rule (Domain I/O Decoupling)", domain pure logic ports must abstract file system concepts away. Currently, they embed `std::path` structures which violate boundary enforcement, leaking infrastructure concerns into the domain boundary.

## Evidence

- source_event: "domain_io_entanglement_structural_arch.md"
  path: "src/domain/ports/fs.rs"
  loc: "FsPort"
  note: "FsPort relies on `std::path::{Path, PathBuf}` for path parameters and returns, coupling the interface strictly to local disk assumptions."
- source_event: "domain_io_entanglement_structural_arch.md"
  path: "src/domain/ports/identity_store.rs"
  loc: "IdentityStore"
  note: "IdentityStore returns a `std::path::PathBuf` from `identity_path`, tying the abstraction to the file system instead of a generic identifier."
- source_event: "domain_io_entanglement_structural_arch.md"
  path: "crates/mev-internal/src/domain/submodule_path.rs"
  loc: "validate_submodule_path"
  note: "Uses `std::path::Path` to validate submodule paths, even though submodules are inherently Git-specific strings, bleeding OS-level filesystem normalization logic into the domain."

## Change Scope

- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
