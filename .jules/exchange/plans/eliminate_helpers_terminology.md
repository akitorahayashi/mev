---
label: "refacts"
---

## Goal

Rename the ambiguous term "helpers" to clearly describe exact responsibilities in CLI commands and code structures.

## Problem

The term "helpers" is used as a generic help string for internal `Git` and `Gh` subcommands ("Git helpers" and "GitHub CLI helpers"), and as a generic group comment in the backup orchestration module ("Shared helpers"). This hides the true capabilities and groups unrelated functions together.

## Affected Areas

### Rust Domain & App
- `src/app/cli/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/assets/ansible/roles/nodejs/config/common/coder/AGENTS.md` (if modified during the common renaming, it's addressed there, otherwise handled as general string replacement)

## Constraints

- Files and classes must identify single, specific responsibilities.
- The terms `base`, `common`, `core`, `utils`, and `helpers` are strictly avoided.
- Help strings must describe exactly what the subcommand does.

## Risks

- Naming changes might affect user muscle memory for CLI usage or error output.

## Acceptance Criteria

- Help strings for `Git` and `Gh` subcommands describe exactly what they do, omitting the word "helpers" (e.g., "Git automation commands").
- Shared functions in `src/app/commands/backup/mod.rs` are explicitly named for their tasks or moved to dedicated modules, removing "helpers" terminology.
- All references to "helpers" in comments and documentation are resolved.

## Implementation Plan

1. Use `replace_with_git_merge_diff` to update `src/app/cli/mod.rs`. Change the `/// Git helpers.` docstring to `/// Git internal operations.`. Change the `/// GitHub CLI helpers.` docstring to `/// GitHub CLI internal operations.`.
2. Use `replace_with_git_merge_diff` to update `src/app/commands/backup/mod.rs`. Change the comment `// Shared helpers` to `// Path resolution and display operations`.
3. Use `run_in_bash_session` to run `cargo check` and `cargo test` to ensure no syntactical or test failures.
4. Use `run_in_bash_session` to run a comprehensive search (`grep -ri helpers src/`) to ensure no occurrences of "helpers" remain.
