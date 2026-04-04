---
label: "docs"
implementation_ready: true
---

## Goal

Ensure the CLI implementation advertises the shorthands that are taught in the official usage documentation by making them visible in the CLI help output.

## Problem

The documentation in `docs/usage.md` claims that `mev cf dp` is a shorthand for `mev config deploy`. However, the CLI help output for `mev config --help` hides the `dp` alias because the implementation uses `alias = "dp"` instead of `visible_alias = "dp"`. This causes a drift where the documentation teaches a shorthand that the CLI itself does not document or advertise.

## Context

The `mev` CLI relies heavily on aliases (e.g. `cr` for `create`, `mk` for `make`, `cf` for `config`). These are implemented in `src/app/cli/mod.rs` and subcommands. The documentation in `docs/usage.md` lists these aliases as "Shorthand". However, while most commands use `#[command(visible_alias = "...")]`, `src/app/cli/config.rs` uses `#[command(alias = "dp")]`, causing the `dp` alias to be hidden from `mev config --help`.

## Evidence

- path: "docs/usage.md"
  loc: "43"
  note: "Documents `mev cf dp` as a Shorthand for `mev config deploy`."
- path: "src/app/cli/config.rs"
  loc: "11"
  note: "Implements the alias using `#[command(alias = \"dp\")]`, which hides it from the CLI help output unlike other subcommands that use `visible_alias`."

## Change Scope

- `src/app/cli/config.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
