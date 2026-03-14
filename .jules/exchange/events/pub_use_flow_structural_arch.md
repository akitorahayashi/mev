---
label: "refacts"
created_at: "2026-03-14"
author_role: "structural_arch"
confidence: "high"
---

## Problem

The application layer and domain layer are entangled via `pub use` statements and implicit backflow. `src/app/api.rs` exports domain models (`BackupTarget`, `Profile`, `ExecutionPlan`, etc.), and `src/lib.rs` exports both `api` and `cli`. This creates a situation where the application layer is acting as the public facade for domain constructs, which violates boundary sovereignty and directionality.

## Goal

Minimize public surface and enforce unidirectional dependency flow. The domain layer should not be re-exported by the application layer. Instead, entry points (CLI and programmatic API) should reside at the true edge, and the domain should be consumed directly or via deliberate adapter mappings, preventing "export everything" anti-patterns.

## Context

The current `pub use` re-exports in `src/app/api.rs` and `src/lib.rs` mean the internal architecture (Domain -> Ports -> Adapters -> App) is exposed to programmatic consumers via the `app` module. The application layer's responsibility is orchestration, not serving as a domain catalog. This makes refactoring the domain harder because its types are part of the `app::api` contract.

## Evidence

- path: "src/app/api.rs"
  loc: "14-19"
  note: "Re-exports domain models (`BackupTarget`, `AppError`, `ExecutionPlan`, `IdentityState`, `Profile`, `VcsIdentity`) via `pub use crate::domain::...`."

- path: "src/lib.rs"
  loc: "15-18"
  note: "Exports `app::cli::run as cli` and `app::api`. This makes `app` the public interface of the crate."

## Change Scope

- `src/app/api.rs`
- `src/lib.rs`