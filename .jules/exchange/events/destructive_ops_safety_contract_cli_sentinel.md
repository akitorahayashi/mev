---
label: "feats"
created_at: "2023-10-25"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

Destructive commands that mutate system configuration (`create`, `make`, `switch`) lack standardized safety contracts. While they offer flags like `--overwrite`, they proceed automatically without confirmation prompts, `--dry-run` modes, or protective warnings, increasing the risk of unintended system changes.

## Goal

Implement consistent safety contracts for all destructive operations. Add `--dry-run` to preview actions where feasible, or incorporate a unified confirmation prompt/warning strategy (with an explicit override like `--yes` or `--force`) before mutating configurations.

## Context

Running commands like `mev switch` modifies global user `.gitconfig` and JJ settings silently. The `create` and `make` commands similarly trigger broad Ansible state changes. Safety measures should be uniform for operations carrying similar risk levels to prevent operational accidents and user mistakes.

## Evidence

- path: "src/app/cli/create.rs"
  loc: "pub struct CreateArgs { ... }"
  note: "Defines no `--dry-run` or `--yes` option, but executes an Ansible plan that changes the system."
- path: "src/app/cli/make.rs"
  loc: "pub struct MakeArgs { ... }"
  note: "Lacks `--dry-run` or safety confirmation before executing targeted Ansible changes."
- path: "src/app/cli/switch.rs"
  loc: "pub struct SwitchArgs { ... }"
  note: "Silently switches identity profiles and modifies the Git system config without asking for user consent."

## Change Scope

- `src/app/cli/create.rs`
- `src/app/cli/make.rs`
- `src/app/cli/switch.rs`
- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/switch/mod.rs`
