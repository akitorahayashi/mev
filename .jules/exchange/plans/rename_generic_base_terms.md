---
label: "refacts"
---

## Goal

Rename vague usages of "base" and "common" across variables, files, and domains to be specific to their domain context ("config_directory", "repository_root", and "llm_global_models").

## Current State

The terms `base` and `common` are used ambiguously across the repository, hiding their actual domain responsibilities.
- `src/adapters/identity_store.rs`: The module documentation refers to `~/.config/` generically as the "base path" instead of describing its domain role.
- `crates/mev-internal/src/adapters/git.rs`: Uses the variable `base_dir` to represent a Git repository working directory.
- `src/assets/ansible/roles/llm/tasks/mlx.yml`: Uses `llm_common_models_mlx` to refer to global models.
- `src/assets/ansible/roles/llm/tasks/ollama.yml`: Uses `llm_common_models` to refer to global models.

## Plan

1. In `src/adapters/identity_store.rs`, update the module documentation on line 3 to say "The config directory is `~/.config/`" instead of "The base path is `~/.config/`".
2. In `crates/mev-internal/src/adapters/git.rs`, rename the `base_dir` variable in the `remove_submodule_module_dir` function to `repository_root` and update its usage on the following line.
3. In `src/assets/ansible/roles/llm/tasks/mlx.yml`, rename the Ansible variable `llm_common_models_mlx` to `llm_global_models_mlx` throughout the file.
4. In `src/assets/ansible/roles/llm/tasks/ollama.yml`, rename the Ansible variable `llm_common_models` to `llm_global_models` throughout the file.
5. Search the codebase for remnants of the old terms (e.g., `rg "base path"`, `rg "base_dir"`, `rg "llm_common_models"`) to verify deletion.

## Constraints

- Code changes must adhere to the single responsibility principle and accurate domain modeling.
- The modifications are strictly renames to improve clarity and must not break tests or change external behavior.
- Only the identified, localized instances should be modified to avoid regressions in unrelated areas.

## Acceptance Criteria

- The term "base path" is removed from `src/adapters/identity_store.rs`.
- `base_dir` in `crates/mev-internal/src/adapters/git.rs` is changed to `repository_root`.
- `llm_common_models_mlx` and `llm_common_models` are respectively renamed to `llm_global_models_mlx` and `llm_global_models` in the specified Ansible YAML tasks.
- No obsolete terms remain unaddressed at the target locations.
