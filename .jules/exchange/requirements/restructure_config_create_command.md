---
label: "bugs"
implementation_ready: false
---

## Goal

Restructure the `config create` command to conform to standard verb-object forms, extracting `role` into an explicit positional argument or subcommand structure.

## Problem

The `config create` command violates the structural consistency rule (verb + [object] + arguments) by using `create` as a subcommand rather than a verb on an object. Other CLI areas follow better structures, such as `mev identity set` and `mev identity show`. To prevent structural drift and naming vocabulary dispersion, the CLI structure should be evaluated and aligned with established norms.

## Evidence

- source_event: "mandatory_options_config_cli_sentinel.md"
  path: "src/app/cli/config.rs"
  loc: "pub enum ConfigCommand { Create { role: Option<String>, overwrite: bool } }"
  note: "The command structure uses 'config' as the verb/object and 'create' as the action, whereas other areas like 'identity' use 'set'/'show'."
- source_event: "mandatory_options_config_cli_sentinel.md"
  path: "src/app/cli/mod.rs"
  loc: "Config(config::ConfigCommand),"
  note: "Top-level command configuration."

## Change Scope

- `src/app/cli/config.rs`
- `src/app/cli/mod.rs`

## Constraints

- Command designs must audit for structural consistency (verb + object form).
- The CLI command structure must remain backward compatible or document breaking changes.

## Acceptance Criteria

- The `config create` command is updated in `src/app/cli/config.rs` to clearly reflect standard CLI patterns (verb + [object] + arguments).
