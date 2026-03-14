---
label: "refacts"
implementation_ready: false
---

## Goal
Eliminate silent fallbacks (`unwrap_or_default()`, `.unwrap_or()`) and type-erased errors (`Box<dyn Error>`) across internal crates and adapters, ensuring explicit error classification and propagation.

## Problem
The codebase currently swallows failures or collapses them into generic representations. The `mev-internal` library returns `Box<dyn Error>`, dropping contextual boundary information. Additionally, commands, executors, and Git/JJ adapters silently swallow external failures via `unwrap_or`/`unwrap_or_default`, masking real issues like signal terminations or I/O failures.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "error_handling_rustacean.md"
  path: "crates/mev-internal/src/adapters/process.rs"
  loc: "run_status and run_output"
  note: "Returns `Result<..., Box<dyn std::error::Error>>` for process execution failures."
- source_event: "silent_failure_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "152"
  note: "The `ansible-playbook` exit code is fetched using `.unwrap_or(-1)` if killed by signal."
- source_event: "silent_fallbacks_data_arch.md"
  path: "src/adapters/git/cli.rs"
  loc: "read_config"
  note: "Uses `unwrap_or_default()` when the git command fails."

## Change Scope
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `src/app/commands/backup/mod.rs`
- `src/adapters/ansible/executor.rs`
- `src/adapters/git/cli.rs`
- `src/adapters/jj/cli.rs`

## Constraints
- Errors must preserve domain meaning and use explicit typings (e.g. `thiserror`).
- `unwrap_or`/`unwrap_or_default` must be replaced by Result propagation where failure is plausible.

## Acceptance Criteria
- Type-erased `Box<dyn Error>` usages are replaced with strongly-typed enums.
- Git, JJ, Ansible, and process executors properly bubble up errors without defaulting.
