---
label: "docs"
created_at: "2024-03-26"
author_role: "consistency"
confidence: "high"
---

## Problem

The CLI documentation in `docs/usage.md` is missing examples for the `config` (`cf`) and `identity` (`id`) shorthands, resulting in drift between the implemented CLI behavior and the usage manual. Furthermore, `mev backup --list` fails to mention the implemented `-l` short flag.

## Goal

Ensure that `docs/usage.md` fully documents the available shorthands for all core subcommands, explicitly adding `cf` for `config`, `id` for `identity`, and `-l` for `backup --list`.

## Context

The `mev` CLI provides consistent aliases for all subcommands (e.g., `cr`, `mk`, `ls`, `cf`, `id`, `sw`, `bk`, `u`) to improve developer experience. However, `docs/usage.md` fails to document `cf`, `id`, and `-l`, causing an incomplete reference.

## Evidence

- path: "docs/usage.md"
  loc: "39-42"
  note: "Usage examples for `identity` and `config` omit the `id` and `cf` shorthands entirely."

- path: "src/app/cli/mod.rs"
  loc: "48-52"
  note: "The `config` command uses alias `cf` and `identity` uses `id`."

- path: "docs/usage.md"
  loc: "59"
  note: "`mev backup --list` is listed, but missing the shorthand `-l`."

- path: "src/app/cli/backup.rs"
  loc: "15"
  note: "The `--list` flag implements a short option `-l`."

## Change Scope

- `docs/usage.md`
