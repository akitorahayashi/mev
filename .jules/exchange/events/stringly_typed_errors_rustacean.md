---
label: "refacts"
created_at: "2024-05-24"
author_role: "rustacean"
confidence: "high"
---

## Problem
The application relies heavily on `String` variants in `AppError` and returns `Box<dyn std::error::Error>` from container initializers instead of preserving error domain context and type safety.

## Goal
Refactor error handling to avoid stringly-typed errors, provide strongly typed variants, and remove `Box<dyn std::error::Error>` from application boundary methods.

## Context
Error handling should preserve domain meaning after propagation. `AppError::Config(String)`, `AppError::Backup(String)`, and others collapse distinct errors into opaque strings, making matching or programmatic recovery impossible. Additionally, using `Box<dyn std::error::Error>` erases type information entirely at the container initialization boundary.

## Evidence
- path: "src/domain/error.rs"
  loc: "17-25"
  note: "AppError variants like Config, Update, and Backup encapsulate String payloads instead of distinct inner types or enums."
- path: "src/app/container.rs"
  loc: "40, 60"
  note: "DependencyContainer::new and for_identity return Result<Self, Box<dyn std::error::Error>> instead of a specific domain error type."
- path: "src/adapters/ansible/executor.rs"
  loc: "69, 208"
  note: "AnsibleAdapter::new and load_catalog return Result<..., Box<dyn std::error::Error>>."

## Change Scope
- `src/domain/error.rs`
- `src/app/container.rs`
- `src/adapters/ansible/executor.rs`