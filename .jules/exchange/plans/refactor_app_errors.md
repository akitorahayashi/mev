---
label: "refacts"
---

## Goal

Refactor error propagation to use specific, structured types (`ConfigError`, `BackupError`) that retain nested errors and boundary context instead of collapsing failures into generic string messages.

## Current State

Errors are losing domain meaning because specific failures are collapsed into broadly typed string-wrapping variants.
- `src/domain/error.rs`: Defines `Config(String)` and `Backup(String)` which act as generic catch-alls.
- `src/adapters/identity_store.rs`: Discards `serde_json` and IO error structures by converting them into `AppError::Config` strings.
- `src/app/commands/backup/system.rs`: Discards YAML parsing and generic IO failure structures by formatting them into `AppError::Backup` strings.

## Plan

1. Define Ownership Boundaries in Domain Errors: Re-own `AppError::Config` and `AppError::Backup` by defining specific, structured nested error enums (`ConfigError`, `BackupError`) in the domain layer (`src/domain/error.rs`). These enums must retain original source errors (e.g., `serde_json::Error`, `std::io::Error`) and exact context (e.g., file paths, keys) as strongly typed variants instead of strings.
2. Re-own Adapter Error Responses: Update adapter implementations (such as `src/adapters/identity_store.rs`, `src/adapters/macos_defaults.rs`, `src/adapters/vscode.rs`, `src/adapters/git.rs`, `src/adapters/ansible/executor.rs`, and `src/adapters/ansible/runtime_assets.rs`) to instantiate the new structured variants, preserving the exact origin of failures.
3. Re-own Command Orchestration Errors: Update the command layer (`src/app/commands/backup/system.rs`, `src/app/commands/backup/vscode.rs`, `src/app/commands/config/mod.rs`, `src/app/commands/deploy_configs.rs`, `src/app/commands/switch/mod.rs`, `src/app/commands/identity/mod.rs`, `src/app/api.rs`, and `src/app/cli/mod.rs`) to use the new typed domain errors, propagating them up cleanly without string conversion.
4. Preserve Behavior Invariants: Ensure that `AppError` continues to implement `std::fmt::Display` and `std::error::Error`, delegating to the nested errors to ensure the externally logged strings and `source()` chaining are semantically consistent with the previous behavior.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.
- Silent fallbacks are prohibited; any fallback must be explicit.

## Acceptance Criteria

- `AppError::Config` and `AppError::Backup` variants wrap structured data/types (`ConfigError`, `BackupError`) rather than strings.
- `src/adapters/identity_store.rs` and `src/app/commands/backup/system.rs` return structured error variants mapping to the underlying root cause.
- The original error types (e.g., `std::io::Error`, `serde_yaml::Error`) are retrievable via the `std::error::Error::source` trait where appropriate.
- Required tests are updated or passing after the change.