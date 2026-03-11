---
label: "refacts"
---

## Goal

Restructure the `config create` command to conform to standard verb-object forms, extracting `role` into an explicit positional argument or subcommand structure.

## Problem

The `config create` command violates the structural consistency rule (verb + [object] + arguments) by using `create` as a subcommand rather than a verb on an object. Other CLI areas follow better structures, such as `mev identity set` and `mev identity show`. To prevent structural drift and naming vocabulary dispersion, the CLI structure should be evaluated and aligned with established norms.

## Affected Areas

### CLI Module

- `src/app/cli/config.rs`
- `src/app/cli/mod.rs`

## Constraints

- Command designs must audit for structural consistency (verb + object form).
- The CLI command structure must remain backward compatible or document breaking changes.

## Risks

- Breaking existing workflows or scripts that depend on the previous argument ordering or flag usage for `mev config create`.

## Acceptance Criteria

- The `config create` command is updated in `src/app/cli/config.rs` to clearly reflect standard CLI patterns (verb + [object] + arguments).
- Tests must pass.

## Implementation Plan

1. Update `src/app/cli/config.rs` to refactor the `Create` subcommand into a more explicit object-verb structure (e.g., `ConfigCommand::Deploy(RoleConfigArgs)` or extracting `role` into an explicit sub-argument structure) that satisfies the verb-object structural consistency rule.
2. Update `src/app/cli/mod.rs` if `ConfigCommand` usage needs adjustment based on changes made in `src/app/cli/config.rs`.
3. Update `src/app/commands/config/mod.rs` (if necessary) to accept the modified arguments.
4. Run all tests to ensure the CLI remains functional and backward-compatible (or explicitly document breaking changes if a CLI redesign was performed).
