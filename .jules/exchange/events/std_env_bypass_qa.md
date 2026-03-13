---
label: "refacts"
created_at: "2024-05-15"
author_role: "qa"
confidence: "high"
---

## Problem

Application domain orchestration logic uses direct operating system environment access `std::env::var` (like `std::env::var("HOME")` inside `src/app/commands/backup/mod.rs`), which bypasses environment abstractions and prevents reliable sandboxing or injecting alternate environment variables during tests.

## Goal

Remove direct reads from `std::env` inside domain commands to prevent logic from coupling tightly to the host execution environment state. State should be captured implicitly by dependency injection paths (like `local_config_root` resolving HOME previously) or provided explicitly as injected variables through a defined port boundary context.

## Context

Relying on direct `std::env` polling in internal business logic violates dependency injection architectural rules and the principle of determinism. This results in global state that tests cannot control safely across thread executions (like the concurrent Rust runner) without producing race conditions. By ensuring all context resolves via pure logic parameters passed down from the top-level application boundaries, testability and determinism are enforced.

## Evidence

- path: "src/app/commands/backup/mod.rs"
  loc: "format_string"
  note: "Directly calls `std::env::var(\"HOME\")` to format string values, binding the application logic to the host machine's state without dependency inversion."

## Change Scope

- `src/app/commands/backup/mod.rs`