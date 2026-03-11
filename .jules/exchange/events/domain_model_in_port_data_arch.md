---
label: "refacts"
created_at: "2026-03-11"
author_role: "data_arch"
confidence: "medium"
---

## Problem

The domain type `IdentityState` (representing the persistable model of identities) is defined in the `src/domain/ports/identity_store.rs` file, coupling a core domain state struct with a port (interface) definition.

## Goal

Separate domain models from port definitions to uphold the Single Source of Truth and Boundary Sovereignty principles.

## Context

Domain models should define the core concepts and state representations. Ports should only define the interfaces that adapters must implement to interact with the core domain. Placing `IdentityState` in `src/domain/ports/identity_store.rs` muddles the boundary between domain data structures and port interfaces. The `IdentityState` struct conceptually belongs with the rest of the identity models, likely in `src/domain/vcs_identity.rs` or a dedicated module.

## Evidence

- path: "src/domain/ports/identity_store.rs"
  loc: "27-31"
  note: "`IdentityState` struct is defined alongside the `IdentityStore` trait."
- path: "src/domain/vcs_identity.rs"
  loc: "1-9"
  note: "Other VCS identity related models like `VcsIdentity` are defined here."

## Change Scope

- `src/domain/ports/identity_store.rs`
- `src/domain/vcs_identity.rs`