---
label: "refacts"
implementation_ready: false
---

## Goal

Separate domain models from port definitions to uphold Boundary Sovereignty and ensure data structures are isolated from adapter contracts.

## Problem

The domain type `IdentityState` (representing the persistable model of identities) is defined in the `src/domain/ports/identity_store.rs` file. This couples a core domain state representation with a port (interface) definition, blurring the boundary between pure data models and external boundaries.

## Evidence

- source_event: "domain_model_in_port_data_arch.md"
  path: "src/domain/ports/identity_store.rs"
  loc: "27-31"
  note: "`IdentityState` struct is defined alongside the `IdentityStore` trait."
- source_event: "domain_model_in_port_data_arch.md"
  path: "src/domain/vcs_identity.rs"
  loc: "1-9"
  note: "Other VCS identity related models like `VcsIdentity` are defined here, which is a more appropriate conceptual home."

## Change Scope

- `src/domain/ports/identity_store.rs`
- `src/domain/vcs_identity.rs`

## Constraints

- Core domain models (state representations) must not be defined within port interface files to uphold Boundary Sovereignty and separate data structures from adapter contracts.

## Acceptance Criteria

- The `IdentityState` model is moved out of `src/domain/ports/identity_store.rs` and placed in `src/domain/vcs_identity.rs` or a similarly dedicated module.
- `src/domain/ports/identity_store.rs` contains only the `IdentityStore` trait.
