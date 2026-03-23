---
label: "refacts"
---

## Goal

Refactor the codebase to adhere to architecture naming conventions by replacing ambiguous directory and code names (e.g., "core", "helpers") with specific, capability-driven names. Correct documentation regarding role profiles to match actual repository state.

## Current State

- `docs/architecture.md`: Explicitly forbids "core/" and "helpers/" directories/names but these exist. It also wrongly states `config/profiles/` is used by "brew only", whereas the `llm` role clearly uses it.
- `src/assets/ansible/roles/shell/config/common/alias/core/`: Contains `open.sh` and `unix.sh` scripts. Violates the strict naming rule against "core/".
- `src/app/commands/backup/mod.rs`: Uses the comment section `// Shared helpers` (line 270) to delineate utility functions, violating the rule against using ambiguous names like "helpers".
- `crates/mev-internal/src/testing/env_mock.rs`: The module doc comment uses the phrase `//! Test helpers for mocking the environment.`, violating the prohibition of "helpers".
- `src/app/cli/mod.rs`: Clap command documentation describes Git and GitHub subcommands using the term "helpers", which is ambiguous and banned.
- `crates/mev-internal/src/app/cli/mod.rs`: Same issue as `src/app/cli/mod.rs`, uses "helpers" in command documentation strings.

## Plan

1. Rename `src/assets/ansible/roles/shell/config/common/alias/core/` to `src/assets/ansible/roles/shell/config/common/alias/system/`.
   - Verify the rename was successful by listing the parent directory: `ls src/assets/ansible/roles/shell/config/common/alias/`.
2. Update `docs/architecture.md` line 81 from `- Roles store configs in config/common/ (all roles) and config/profiles/ (brew only)` to `- Roles store configs in config/common/ (all roles) and config/profiles/ (e.g., brew, llm)`.
   - Verify the update using `cat -n docs/architecture.md | sed -n '80,85p'`.
3. Update `src/app/commands/backup/mod.rs` replacing the comment `// Shared helpers` with `// Directory resolution`.
   - Verify the change using `cat -n src/app/commands/backup/mod.rs | sed -n '265,275p'`.
4. Update `crates/mev-internal/src/testing/env_mock.rs` replacing `//! Test helpers for mocking the environment.` with `//! Environment mocking for tests.`
   - Verify the change using `cat -n crates/mev-internal/src/testing/env_mock.rs | sed -n '1,5p'`.
5. Update `src/app/cli/mod.rs` replacing `/// Git helpers.` with `/// Git operations.` and `/// GitHub CLI helpers.` with `/// GitHub CLI operations.`
   - Verify the change using `cat -n src/app/cli/mod.rs | sed -n '65,75p'`.
6. Update `crates/mev-internal/src/app/cli/mod.rs` replacing `/// Git helpers.` with `/// Git operations.` and `/// GitHub CLI helpers.` with `/// GitHub CLI operations.`
   - Verify the change using `cat -n crates/mev-internal/src/app/cli/mod.rs | sed -n '15,30p'`.
7. Search the repository for CLI test assertions that use the word "helpers" (e.g. `rg "helpers"`) and update any that relied on the old `gh` or `git` help text.
   - Verify changes using `cargo test` and `cd crates/mev-internal && cargo test`.
8. Run all tests with `cargo test` and `cd crates/mev-internal && cargo test` to ensure no regressions.

## Acceptance Criteria

- `src/assets/ansible/roles/shell/config/common/alias/core` is renamed to `system`.
- `docs/architecture.md` no longer claims `config/profiles/` is `(brew only)`.
- No references to "helpers" exist in `src/app/commands/backup/mod.rs`, `crates/mev-internal/src/testing/env_mock.rs`, `src/app/cli/mod.rs`, or `crates/mev-internal/src/app/cli/mod.rs`.
- `cargo test` passes in root and `crates/mev-internal/`.

## Risks

- Changing CLI doc comments (`///`) alters `--help` output, which will break tests asserting on exact CLI help text. These must be caught and updated.