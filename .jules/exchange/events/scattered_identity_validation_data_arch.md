---
label: "refacts"
created_at: "2026-03-14"
author_role: "data_arch"
confidence: "high"
---

## Problem

The `VcsIdentity` model allows invalid empty states, causing validation logic to be scattered to application layer call sites.

## Goal

Enforce invariants at the boundary by using appropriate types (e.g., a non-empty string type or a constructor that returns an error) so invalid states cannot be represented.

## Context

The First Principles of data architecture require representing valid states only, encoding invariants so invalid states are hard to express. The current `VcsIdentity` uses raw `String` types for name and email, allowing empty strings. This leads to missing validation in the domain, forcing the application layer to manually check for empty strings before use.

## Evidence

- path: "src/domain/vcs_identity.rs"
  loc: "VcsIdentity"
  note: "Defines name and email as raw strings without validation."
- path: "src/app/commands/switch/mod.rs"
  loc: "execute"
  note: "Call site manually validates that vcs_identity.name and vcs_identity.email are not empty, proving invariants are not enforced by the type."

## Change Scope

- `src/domain/vcs_identity.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/identity/mod.rs`
