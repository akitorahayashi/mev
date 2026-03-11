---
label: "bugs"
created_at: "2026-03-11"
author_role: "cli_sentinel"
confidence: "medium"
---

## Problem

The `config create` command violates the structural consistency rule (verb + [object] + arguments) by using `create` as a subcommand rather than a verb on an object.

## Goal

Restructure the `config create` command to conform to standard verb-object forms, potentially extracting `role` into an explicit positional argument or subcommand structure aligned with `identity set`/`identity show`.

## Context

Commands should follow standard sentence structures. The CLI uses `mev identity set` and `mev identity show` for configuration tasks, while `mev config create` acts more as an imperative verb. To prevent structural drift and naming vocabulary dispersion, the CLI structure should be evaluated and aligned with established norms.

## Evidence

- path: "src/app/cli/config.rs"
  loc: "pub enum ConfigCommand { Create { role: Option<String>, overwrite: bool } }"
  note: "The command structure uses 'config' as the verb/object and 'create' as the action, whereas other areas like 'identity' use 'set'/'show'."
- path: "src/app/cli/mod.rs"
  loc: "Config(config::ConfigCommand),"
  note: "Top-level command configuration."

## Change Scope

- `src/app/cli/config.rs`
- `src/app/cli/mod.rs`
