---
label: "docs"
created_at: "2024-03-19"
author_role: "consistency"
confidence: "high"
---

## Problem

The CLI help text obscures subcommand shorthands (like `cr` for `create`, `mk` for `make`), making the official documentation (`docs/usage.md`) appear inaccurate or disjointed from the actual application interface.

## Goal

Ensure that subcommand shorthands heavily advertised in `docs/usage.md` are discoverable directly from the CLI help text.

## Context

`docs/usage.md` frequently uses and advertises aliases like `mev cr mbk`, `mev mk vscode`, `mev bk system`, `mev sw p`, and `mev u`. While these are fully implemented in `src/app/cli/mod.rs` using `#[command(alias = "...")]`, the clap framework's default behavior for `alias` does not list them in the generated help text (unlike `visible_alias`). This causes a drift between the user's documented learning path and the CLI's self-documentation. If a user learns about the CLI from `mev --help`, they will never discover the shorthands. If they read `docs/usage.md` and then type `mev --help` to verify, they may think the documentation is outdated or incorrect.

## Evidence

- path: "docs/usage.md"
  loc: "8, 23, 50, 51, 60, 67"
  note: "Usage docs advertise aliases: `mev cr`, `mev mk`, `mev sw`, `mev bk`, `mev u`."

- path: "src/app/cli/mod.rs"
  loc: "28-56"
  note: "Implements `alias` for subcommands but does not use `visible_alias`, hiding them from the `--help` output."

## Change Scope

- `src/app/cli/mod.rs`
