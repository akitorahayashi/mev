---
label: "refacts"
created_at: "2024-04-04"
author_role: "rustacean"
confidence: "high"
---

## Problem

Errors are losing domain meaning and explicit context because they collapse specific failures into broadly typed variants (`AppError::Config` and `AppError::Backup`) wrapping only a `String` message.

## Goal

Refactor error propagation to use more specific types or variants that retain structured data (like the failed path, operation, or nested error types) rather than collapsing everything into strings. This ensures actionable diagnosis and boundary context retention.

## Context

In `src/domain/error.rs`, variants like `AppError::Config(String)` and `AppError::Backup(String)` are used heavily throughout the codebase (`src/adapters/identity_store.rs`, `src/app/commands/backup/system.rs`, `src/app/commands/config/mod.rs`, etc.) to wrap arbitrary error strings via `format!`. This practice of "stringification" discards the original error type and makes programmatic error handling or specific logging impossible, reducing the semantic value of the error enum.

## Evidence

- path: "src/domain/error.rs"
  loc: "11-12"
  note: "Defines `Config(String)` and `Backup(String)` which are used as catch-alls."

- path: "src/adapters/identity_store.rs"
  loc: "48, 62, 67, 70"
  note: "Converts `serde_json` and IO errors into `AppError::Config` strings using `format!`."

- path: "src/app/commands/backup/system.rs"
  loc: "38, 46, 86, 118, 184"
  note: "Uses `format!` to collapse YAML parsing and generic IO failures into `AppError::Backup` strings."

## Change Scope

- `src/domain/error.rs`
- `src/adapters/identity_store.rs`
- `src/app/commands/backup/system.rs`
