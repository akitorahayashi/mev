---
label: "bugs"
---

## Goal

Migrate all operational logs, progress messages, and non-result output to `stderr` (e.g., using `eprintln!`), reserving `stdout` strictly for command result data.

## Problem

Operational logs, progress updates, and success messages are written to `stdout` instead of `stderr`, mixing diagnostic information with result data. Commands like `create`, `make`, `backup`, and `update` use `println!` for logging progress and status updates. This mixes streams and causes automation failures, as piped output will include non-data logs rather than pure result streams.

## Affected Areas

### CLI Commands

- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/commands/update/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`

## Constraints

- Operational logs and progress updates must be written to `stderr` (e.g., using `eprintln!`) rather than `stdout` to separate I/O streams and prevent breaking automation pipelines.
- Standard output (`stdout`) should exclusively carry the output data result of commands.

## Risks

- Changing output streams could break existing brittle scripts that expect operational logs on `stdout` instead of `stderr`.

## Acceptance Criteria

- All `println!` usage for diagnostic or progress output in the specified commands is replaced with `eprintln!`.
- Piping the commands listed in the scope through command line utilities cleanly outputs data without operational message pollution on `stdout`.

## Implementation Plan

1. Replace `println!` with `eprintln!` in `src/app/commands/create/mod.rs`.
   - Update `run` function to use `eprintln!` for progress, completion, and optional steps.
2. Replace `println!` with `eprintln!` in `src/app/commands/make/mod.rs`.
   - Update `run` function to use `eprintln!` for tags, profile, and completion output.
3. Replace `println!` with `eprintln!` in `src/app/commands/backup/mod.rs`.
   - Update `run` and specific backup target implementations to use `eprintln!` for status logs.
   - Ensure the generated default files paths and VSCode extensions list paths are also logged to `stderr`.
4. Replace `println!` with `eprintln!` in `src/app/commands/update/mod.rs`.
   - Update `run` function to use `eprintln!` for current version, upgrade process, and completion.
5. Replace `println!` with `eprintln!` in `src/app/commands/deploy_configs.rs`.
   - Update `deploy_role_configs` to use `eprintln!` for deployment progress logs.
6. Replace `println!` with `eprintln!` in `src/app/commands/config/mod.rs`.
   - Update `run` function to use `eprintln!` for found configurations, existence checks, and deployment completion.
7. Run all tests to ensure the changes do not break any functionality.
   - Run `just test` to verify no regressions in standard behavior or output capturing.
8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
