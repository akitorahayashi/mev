---
label: "refacts"
created_at: "2026-03-14"
author_role: "cli_sentinel"
confidence: "medium"
---

## Problem

The `mev config create` command uses the verb `create` for an action that copies/deploys existing configuration files, introducing a naming synonym conflict with the top-level `mev create` command which provisions a full environment.

## Goal

Rename the subcommand to accurately reflect its deployment action without overlapping with top-level environment creation verbs (e.g., `mev config deploy`).

## Context

CLI naming consistency rules require that "verb and object vocabulary follows established conventions without synonyms". The internal module is named `deploy_configs.rs` and the help text states "Deploy role configs...", confirming that the action is a deployment, not a creation from scratch.

## Evidence

- path: "src/app/cli/config.rs"
  loc: "ConfigCommand::Create"
  note: "The enum variant and CLI command are named 'Create', but the docstring states 'Deploy role configs'."

## Change Scope

- `src/app/cli/config.rs`
