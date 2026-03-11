---
label: "docs"
implementation_ready: true
---

## Goal

Update the documentation in `docs/usage.md` to accurately reflect all implemented CLI aliases, including command aliases and backup target aliases.

## Problem

The documentation in `docs/usage.md` is missing several CLI aliases that are defined in the implementation, and incorrectly lists some that are defined differently. Specifically, aliases for `config` (`cf`), `identity` (`id`), and `config create` (`cr`) are either missing or incorrect. Additionally, the valid backup target alias `vscode-extensions` is not documented alongside `vscode`.

## Evidence

- source_event: "cli_aliases_not_documented_consistency.md"
  path: "docs/usage.md"
  loc: "line 29"
  note: "Under Configuration, it lists `mev identity set`, `mev config create`, etc. but omits aliases `id` and `cf` respectively."
- source_event: "cli_aliases_not_documented_consistency.md"
  path: "src/app/cli/mod.rs"
  loc: "line 36, 40"
  note: "Defines `#[command(alias = \"cf\", subcommand)] Config(...)` and `#[command(alias = \"id\", subcommand)] Identity(...)`."
- source_event: "backup_target_aliases_not_documented_consistency.md"
  path: "src/domain/backup_target.rs"
  loc: "line 15"
  note: "Defines `\"vscode\" | \"vscode-extensions\" => Some(Self::Vscode)`."
- source_event: "backup_target_aliases_not_documented_consistency.md"
  path: "docs/usage.md"
  loc: "lines 45-48"
  note: "Only lists `mev backup vscode` and omits the alias `vscode-extensions`."

## Change Scope

- `docs/usage.md`

## Constraints

- Documentation must conform to the implementation, not vice-versa.
- Declarative updates preserve existing content and integrate new material without duplication or complete replacement.

## Acceptance Criteria

- `docs/usage.md` includes the `cf` alias for the `config` command and the `id` alias for the `identity` command.
- `docs/usage.md` includes the `cr` alias for the `config create` command.
- `docs/usage.md` documents the `vscode-extensions` alias as an alternative for `vscode` under the `backup` command documentation.
