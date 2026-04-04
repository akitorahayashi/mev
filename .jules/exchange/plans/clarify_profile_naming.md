---
label: "feats"
---

## Goal

Separate the "Hardware Target" concept from the "Global Configuration" scope by introducing a `TargetScope` that encapsulates either a `HardwareTarget` (Macbook, MacMini) or a `Global` scope, and rename instances of "Profile" to "Target" to clarify domain terminology.

## Current State

The `Profile` enum in `src/domain/profile.rs` currently mixes hardware variants (`Macbook`, `MacMini`) with a `Global` configuration state. The term "Profile" is used inconsistently to mean both the physical machine target and a global configuration scope.
- `src/domain/profile.rs`: Defines `pub enum Profile { Macbook, MacMini, Global }` which mixes hardware and global scope, and contains validation logic that checks for hardware profiles.
- `src/app/api.rs`: Uses `Profile` for both `create` (hardware only) and `make` (hardware or global) commands.
- `src/domain/execution_plan.rs`: Uses `Profile` as the target for execution.

## Plan

1. Rename `src/domain/profile.rs` to `src/domain/target.rs`.
2. Update `src/domain/target.rs` to define `pub enum HardwareTarget { Macbook, MacMini }` and `pub enum TargetScope { Hardware(HardwareTarget), Global }`.
3. Implement string conversion and validation for both `HardwareTarget` and `TargetScope` in `src/domain/target.rs`.
4. Update `src/domain/error.rs` to replace `InvalidProfile` with `InvalidTarget`.
5. Update `src/domain/mod.rs` to expose `pub mod target;` instead of `pub mod profile;`.
6. Refactor `src/domain/execution_plan.rs` to use `TargetScope` instead of `Profile`.
7. Refactor `src/app/api.rs`, `src/app/cli/create.rs`, and `src/app/cli/make.rs` to accept and validate `HardwareTarget` and `TargetScope` accordingly.
8. Refactor `src/app/commands/create/mod.rs` and `src/app/commands/make/mod.rs` to handle the new target types.
9. Refactor `src/app/commands/list/mod.rs` to display targets instead of profiles.
10. Rename Ansible profile directories from `profiles` to `targets`: `src/assets/ansible/roles/llm/config/profiles` -> `src/assets/ansible/roles/llm/config/targets` and `src/assets/ansible/roles/brew/config/profiles` -> `src/assets/ansible/roles/brew/config/targets`.
11. Update references to the `profile` variable in Ansible task files (`src/assets/ansible/roles/brew/tasks/cask.yml`, `src/assets/ansible/roles/brew/tasks/formulae.yml`, `src/assets/ansible/roles/llm/tasks/mlx.yml`, `src/assets/ansible/roles/llm/tasks/ollama.yml`, `src/assets/ansible/roles/rust/tasks/platform.yml`, `src/assets/ansible/roles/shell/tasks/main.yml`, `src/assets/ansible/roles/shell/config/global/alias/go.sh`, `src/assets/ansible/roles/editor/config/global/settings.json`) to use `target`.
12. Update `src/adapters/ansible/executor.rs` and `src/domain/ports/ansible.rs` to take a target string instead of a profile string.
13. Search for any remnants of the word "profile" using `rg -i profile src/` and `rg -i profile src/assets/ansible/` to ensure full migration.

## Constraints

- Code changes must adhere to single responsibility and accurate domain modeling.
- Ensure all tests are updated to match the new naming and structures.

## Acceptance Criteria

- The term `Profile` is completely removed from the domain modeling and replaced by `HardwareTarget` and `TargetScope`.
- `create` commands strictly require a hardware target.
- `make` commands accept either a hardware target or the global scope.
- All tests pass and there are no instances of the word `profile` left referring to the old domain concept.
