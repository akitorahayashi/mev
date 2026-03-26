---
label: "docs"
implementation_ready: false
---

## Goal

Ensure `docs/usage.md` documents aliases (`cf`, `id`) and short flags (`-l`) consistently with the CLI implementation.

## Problem

The CLI documentation in `docs/usage.md` is missing examples for the `config` (`cf`) and `identity` (`id`) shorthands, resulting in drift between the implemented CLI behavior and the manual. It also misses the `-l` short flag for `backup --list`.

## Evidence

- source_event: "cli_aliases_consistency.md"
  path: "docs/usage.md"
  loc: "39-42"
  note: "Usage examples for `identity` and `config` omit the `id` and `cf` shorthands entirely."

## Change Scope

- `docs/usage.md`

## Constraints

- Ensure all subcommands listed are synchronized with the actual CLI aliases.

## Acceptance Criteria

- `cf`, `id`, and `-l` are explicitly documented in the usage manual.