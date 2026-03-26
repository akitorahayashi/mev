---
label: "refacts"
created_at: "2024-03-26"
author_role: "data_arch"
confidence: "high"
---

## Problem

Some top-level application container initialization functions return `Box<dyn std::error::Error>` instead of the application's typed `AppError`.

## Goal

Align application container dependencies and error returns to use `AppError` instead of generic error boxes.

## Context

While `src/domain/error.rs` exists and implements `AppError`, some initialization code and container configurations return `Box<dyn std::error::Error>`, which breaks the typed error invariant at the boundary layer.

## Evidence

- path: "src/app/container.rs"
  loc: "DependencyContainer::new()"
  note: "`DependencyContainer::new` returns `Result<Self, Box<dyn std::error::Error>>` instead of `Result<Self, AppError>`."

- path: "src/app/container.rs"
  loc: "DependencyContainer::for_identity()"
  note: "`DependencyContainer::for_identity` returns `Result<Self, Box<dyn std::error::Error>>` instead of `Result<Self, AppError>`."

## Change Scope

- `src/app/container.rs`
- `src/adapters/ansible/executor.rs`
