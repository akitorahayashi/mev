---
label: "refacts"
implementation_ready: false
---

## Goal

Rename vague usages of "base" and "common" across variables, files, and domains to be specific to their domain. "base path" should become "config_directory" or "root_path". "base_dir" should be "repository_root". "llm_common_models" should be "llm_default_models" or "llm_global_models".

## Problem

The term `base` is used vaguely in multiple contexts:
1. "base path" (`src/adapters/identity_store.rs:1: //! The base path is ~/.config/...`)
2. "base_dir" (`crates/mev-internal/src/adapters/git.rs`)
3. Generic model layer attributes `llm_common_models_mlx`, etc (`src/assets/ansible/roles/llm/tasks/mlx.yml`)
4. Generic model layer attributes `llm_common_models`, etc (`src/assets/ansible/roles/llm/tasks/ollama.yml`)

These names hide their responsibility and violate "One Concept, One Preferred Term". The principles explicitly say "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers."

## Context

The principles require domain language first, no generic names, and explicitly prohibit "base" and "common".

## Evidence

- path: "src/adapters/identity_store.rs"
  loc: "line 1"
  note: "Refers to '~/.config' as 'base path'."

- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "line 31"
  note: "Uses 'base_dir' for what is a repository working directory."

- path: "src/assets/ansible/roles/llm/tasks/mlx.yml"
  loc: "line 10: `name: llm_common_models_mlx`"
  note: "Uses 'common' inappropriately as a variable root."

- path: "src/assets/ansible/roles/llm/tasks/ollama.yml"
  loc: "line 17: `name: llm_common_models`"
  note: "Uses 'common' inappropriately as a variable root."

## Change Scope

- `src/adapters/identity_store.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `src/assets/ansible/roles/llm/tasks/mlx.yml`
- `src/assets/ansible/roles/llm/tasks/ollama.yml`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
