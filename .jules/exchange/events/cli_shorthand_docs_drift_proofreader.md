---
label: "docs"
created_at: "2024-04-04"
author_role: "proofreader"
confidence: "high"
---

## Problem

The documentation in `docs/usage.md` claims that `mev cf dp` is a shorthand for `mev config deploy`. However, the CLI help output for `mev config --help` hides the `dp` alias because the implementation uses `alias = "dp"` instead of `visible_alias = "dp"`. This causes a drift where the documentation teaches a shorthand that the CLI itself does not document or advertise.

## Goal

Ensure the CLI implementation advertises the shorthands that are taught in the official usage documentation by making them visible in the CLI help output.

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
