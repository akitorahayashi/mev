---
label: "refacts"
implementation_ready: true
---

## Goal

Encode invariants at the boundaries so that invalid states (empty identities) are hard or impossible to express in the core domain model.

## Problem

The `Identity` model allows invalid states (empty strings for name and email) by relying on a runtime `is_configured` check instead of enforcing invariants via types.

## Context

First Principles dictate "Represent Valid States Only: encode invariants so invalid states are hard to express." The current `Identity` model allows instantiation with empty strings and defers validation to `is_configured()`, meaning call sites might process an unconfigured identity.

## Evidence

- path: "src/domain/identity.rs"
  loc: "Identity struct"
  note: "`Identity` struct has `name` and `email` as `String`, allowing empty values, with a separate `is_configured` method."
- path: "src/app/commands/switch/mod.rs"
  loc: "22"
  note: "The `switch::execute` command has to manually check `!identity_config.is_configured()` and fail, demonstrating validation scattered to call sites."

## Change Scope

- `src/domain/identity.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/identity/mod.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
