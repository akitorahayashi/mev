---
label: "docs"
created_at: "2026-03-11"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation in `docs/usage.md` is missing some CLI aliases defined in the implementation, and incorrectly lists some that are defined differently.

## Goal

Update the documentation to accurately reflect the implemented CLI aliases.

## Context

The `docs/usage.md` file defines several shorthand commands (aliases). However, some implemented aliases are missing, and some are incorrect.
- `mev list` alias `mev ls` is documented and implemented.
- `mev create` alias `mev cr` is documented and implemented.
- `mev make` alias `mev mk` is documented and implemented.
- `mev switch` alias `mev sw` is documented and implemented.
- `mev backup` alias `mev bk` is documented and implemented.
- `mev update` alias `mev u` is documented and implemented.
- `mev config` alias `mev cf` is implemented but NOT documented.
- `mev identity` alias `mev id` is implemented but NOT documented.
- `mev config create` is documented with `mev config create` but its alias is `mev config cr` inside the subcommand.

## Evidence

- path: "docs/usage.md"
  loc: "line 29"
  note: "Under Configuration, it lists `mev identity set`, `mev config create`, etc. but omits aliases `id` and `cf` respectively."
- path: "src/app/cli/mod.rs"
  loc: "line 36, 40"
  note: "Defines `#[command(alias = \"cf\", subcommand)] Config(...)` and `#[command(alias = \"id\", subcommand)] Identity(...)`."

## Change Scope

- `docs/usage.md`
