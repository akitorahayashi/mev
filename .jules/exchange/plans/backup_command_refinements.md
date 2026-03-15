---
label: "tests"
---

## Goal

Improve the `backup` command's CLI structure by removing the `--list` flag anti-pattern, ensure correct IO separation by routing warnings to `stderr`, and implement robust tests asserting externally observable behavior to cover the `execute_system` and `execute_vscode` orchestrator logic.

## Current State

- `src/app/cli/backup.rs`: The CLI definition uses a `--list` flag alongside an optional `target` positional argument wrapped in an `ArgGroup`. This breaks standard CLI parsing conventions (where `list` should be a subcommand or specific positional keyword).
- `src/app/commands/backup/mod.rs`: The command uses `println!` to emit a warning when local definitions are not found ("Local definitions not found at..."), polluting `stdout` instead of using `stderr` (`eprintln!`). Furthermore, the orchestrator logic (`execute_system` and `execute_vscode`) lacks automated tests, currently resting at an 8.5% line coverage.
- `tests/cli/backup.rs`: Existing tests only check basic argument parsing (e.g., `--list`, `--help`, unknown target) but fail to assert the actual outcomes, file generation, or stream separation of the command logic.

## Plan

1. Modify `src/app/cli/backup.rs` to replace the `--list` flag with robust positional parsing. The `BackupArgs` struct will define a single `target` argument, and if the user provides `"list"`, the command will list available targets. Remove the `clap::ArgGroup`.
2. Modify `src/app/commands/backup/mod.rs` to change the `println!` call for "Local definitions not found at {}. Using package defaults." to `eprintln!`, ensuring non-data logs are strictly output to `stderr`.
3. Update `tests/cli/backup.rs` to remove tests for `--list`, `-l`, and `--ls`, and replace them with tests for the positional `list` command (e.g., `backup list`).
4. Add integration tests in `tests/cli/backup.rs` to assert the observable behavior of `backup system` and `backup vscode`. These tests must:
   - Verify that `system` backup generates the expected YAML output (`system.yml`) in the file system by checking file contents.
   - Verify that missing local definitions properly fallback to package defaults and emit a warning to `stderr` without polluting `stdout`.
   - Verify that `vscode` backup retrieves extensions and generates the expected JSON file (`vscode-extensions.json`).

## Acceptance Criteria

- `--list` flag is completely removed and replaced by positional parsing (e.g., `backup list`).
- Missing local definitions fallback warning is routed to `stderr`.
- Coverage for `execute_system` and `execute_vscode` is significantly increased by asserting externally observable outcomes (files written, `stdout`/`stderr` outputs) at the CLI boundary.
- Tests do not mock internal orchestrator state, relying entirely on the `TestContext` boundary (file system and dependency ports).

## Risks

- CLI structure changes might break existing scripts or user expectations that rely on the `--list` flag.
- Setting up the `TestContext` for `system` backup may require precise fixture data to ensure determinism across different environments.
