---
label: "refacts"
implementation_ready: false
---

## Goal

Standardize terminology by replacing the generic term "target" with more specific domain nouns where appropriate to prevent conceptual overloading.

## Problem

The word `target` is heavily overloaded across the codebase. It represents at least three distinct concepts:
1. A GitHub repository to act upon (`repo_target.rs`).
2. A generic CLI argument value for Backup components (`backup target`).
3. A destination directory when moving or copying files (`deploy_configs.rs`).
4. The cargo compilation output directory (`tests/harness/test_context.rs`).

## Context

The guiding principles mandate that domain models must not be generic and should follow 'One Concept, One Preferred Term'. Generic terms like "target" should be avoided across different contexts, especially when dealing with specific domain entities like Repositories or Components.

## Evidence

- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "line 1: `//! Repository target resolution.`"
  note: "Uses 'target' to describe a repository."

- path: "src/app/cli/backup.rs"
  loc: "line 19: `pub target: Option<String>,`"
  note: "Uses 'target' as the CLI struct field for what is functionally a 'BackupComponent'."

- path: "src/app/commands/deploy_configs.rs"
  loc: "line 37: `let target = local_config_root.join(role);`"
  note: "Uses 'target' to represent a file system destination directory."

- path: "tests/harness/test_context.rs"
  loc: "line 18: `let test_tmp_dir = std::path::Path::new(manifest_dir).join(\"target\").join(\"test_tmp\");`"
  note: "Standard cargo build directory usage."

## Change Scope

- `crates/mev-internal/src/domain/repo_target.rs`
- `src/app/cli/backup.rs`
- `src/app/commands/backup/system.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
