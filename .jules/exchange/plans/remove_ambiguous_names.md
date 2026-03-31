---
label: "refacts"
---

## Goal

Identify and rename instances of `utils`, `helper`, `base`, `common`, and `core` to names that describe their specific responsibilities, eliminating ambiguous module and variable names.

## Current State

- `src/adapters/identity_store/paths.rs`: Contains the function `config_base()`, which uses the ambiguous term `base`.
- `src/assets/ansible/roles/shell/config/global/alias/dev/dev.sh`: Contains variables and comments using ambiguous terms like `base`, `helper`, and `common`.
- `src/assets/ansible/roles/nodejs/config/global/coder/skills/svo-cli-design/SKILL.md`: Uses the ambiguous term `core` in a heading (`## Core Objective`) and description (`core required inputs`).
- `src/assets/ansible/roles/nodejs/config/global/coder/skills/effective-prompting/SKILL.md`: Uses the ambiguous term `core` in a heading (`## Core Objective`).

## Plan

1. Rename the private function `config_base()` to `config_dir()` in `src/adapters/identity_store/paths.rs`. Update the references to `config_base` within `default_identity_path` and `local_config_root` to use `config_dir`.
2. Modify `src/assets/ansible/roles/shell/config/global/alias/dev/dev.sh`.
   - Rename `base_command` to `target_command` (lines 4, 6, 11, 14, 16).
   - Replace the comment `# Helper function to build command with optional run_prefix` with `# Construct command with optional run_prefix`.
   - Replace the comment `# Common development commands` with `# Standard development commands`.
3. Modify `src/assets/ansible/roles/nodejs/config/global/coder/skills/svo-cli-design/SKILL.md`.
   - Replace the heading `## Core Objective` with `## Primary Objective`.
   - Replace the phrase `core required inputs` with `primary required inputs`.
4. Modify `src/assets/ansible/roles/nodejs/config/global/coder/skills/effective-prompting/SKILL.md`.
   - Replace the heading `## Core Objective` with `## Primary Objective`.
5. Run workspace tests
   - Run `cargo test` to verify changes haven't caused regressions.
6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. Submit the changes.
