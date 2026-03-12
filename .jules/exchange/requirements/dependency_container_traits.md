---
label: "refacts"
implementation_ready: false
---

## Goal

Refactor `DependencyContainer` to hold trait objects for port dependencies instead of concrete adapters, enabling unit testing of orchestration logic.

## Problem

`DependencyContainer` currently binds directly to concrete adapters (e.g., `GitCli`, `IdentityFileStore`). This prevents injecting in-memory test doubles to test command orchestration logic (`src/app/commands/`), forcing reliance on slow, side-effect-heavy integration tests.

## Evidence

- source_event: "untestable_app_layer_qa.md"
  path: "src/app/container.rs"
  loc: "line 17-26"
  note: "`DependencyContainer` struct fields are typed to concrete adapters (`pub git: GitCli`) instead of traits (`pub git: Box<dyn GitPort>`)."

## Change Scope

- `src/app/container.rs`
- `src/app/api.rs`

## Constraints

- Container construction should be capable of accepting test doubles without exposing generic pollution or complicated lifetimes where possible.

## Acceptance Criteria

- `DependencyContainer` fields use trait objects or bounded generics.
- It is possible to instantiate `DependencyContainer` with test doubles from `src/testing/`.
