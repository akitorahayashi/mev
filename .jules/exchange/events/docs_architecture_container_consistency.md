---
label: "docs"
created_at: "2024-03-14"
author_role: "consistency"
confidence: "high"
---

## Problem

The architectural documentation references `src/app/context.rs` for dependency wiring, but the file is actually named `src/app/container.rs`.

## Goal

Update the architecture documentation to reflect the correct filename (`container.rs`) used for dependency wiring in the application layer.

## Context

The documentation serves as the canonical map for the package structure. When file paths in `docs/architecture.md` do not match the implementation, it creates confusion for contributors navigating the application layer boundaries and dependency injection setup.

## Evidence

- path: "docs/architecture.md"
  loc: "19"
  note: "Documents `├── context.rs          # Dependency wiring (ports → adapters)`"
- path: "src/app/container.rs"
  loc: "1"
  note: "The actual implemented file is `container.rs`, which contains the `DependencyContainer` struct for dependency wiring."

## Change Scope

- `docs/architecture.md`
