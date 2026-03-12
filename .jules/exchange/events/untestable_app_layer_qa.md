---
label: "refacts"
created_at: "2025-01-14"
author_role: "qa"
confidence: "high"
---

## Problem

The application layer (`DependencyContainer`) wires concrete adapter implementations (e.g., `GitCli`, `JjCli`, `IdentityFileStore`) directly to the application context instead of using domain port abstractions (e.g., `Box<dyn GitPort>`). This prevents injecting test doubles to unit-test the command orchestration logic.

## Goal

Refactor `DependencyContainer` to hold trait objects (or use generics) for its port dependencies, enabling the injection of in-memory test doubles (from `src/testing/mod.rs`) so that the command orchestration logic in `src/app/commands/` can be unit tested without side effects.

## Context

The core value of the ports-and-adapters architecture is the ability to test pure logic without side effects. Currently, `src/app/commands/` handles command orchestration (e.g., switching identities, backing up files). However, because `DependencyContainer` relies on concrete types, this logic cannot be unit tested in isolation. We are forced to rely entirely on slow, integration-level tests (`tests/cli/`) that execute real I/O. Testing pure orchestration logic via slow integration tests is an anti-pattern that impacts the feedback-speed design.

## Evidence

- path: "src/app/container.rs"
  loc: "line 17-26"
  note: "`DependencyContainer` struct fields are typed to concrete adapters (`pub git: GitCli`) instead of traits (`pub git: Box<dyn GitPort>`)."

## Change Scope

- `src/app/container.rs`
- `src/app/api.rs`