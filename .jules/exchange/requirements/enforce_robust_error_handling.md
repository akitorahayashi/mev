---
label: "refacts"
implementation_ready: false
---

## Goal

Replace type-erased errors (`Box<dyn std::error::Error>`) with strongly-typed domain errors (like `AppError`) to preserve error semantics, classifications, and enable proper error handling. Ensure all `unwrap()`/`expect()` usage and silent fallback behaviors (like `.unwrap_or(-1)`) outside test modules are replaced with explicit error handling and propagation using robust domain error types.

## Problem

Widespread use of type-erased `Box<dyn std::error::Error>` across the codebase (specifically in adapters) loses semantic error classification. Furthermore, scattered use of `unwrap()`, `expect()`, and silent fallback patterns (like `.unwrap_or(-1)`) outside of tests bypass error boundaries and mask failures, undermining the principle that errors are part of the contract and must retain their semantic meaning.

## Evidence

- source_event: "type_erased_errors_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "line 208"
  note: "`fn load_catalog` returns `Result<Catalog, Box<dyn std::error::Error>>` which erases specifics of parsing vs IO errors."
- source_event: "type_erased_errors_rustacean.md"
  path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "line 8"
  note: "`resolve_repo_ref` returns `Box<dyn std::error::Error>`."
- source_event: "type_erased_errors_rustacean.md"
  path: "crates/mev-internal/src/adapters/process.rs"
  loc: "line 8, 20"
  note: "Process execution errors are boxed."
- source_event: "type_erased_errors_rustacean.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "lines 9, 23, 33, 46"
  note: "Git adapter operations use boxed errors."
- source_event: "type_erased_errors_rustacean.md"
  path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "lines 9, 29, 39"
  note: "GitHub adapter operations return boxed errors."
- source_event: "unwraps_and_silent_fallbacks_rustacean.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "lines 100, 107, 114, 121"
  note: "Tests heavily rely on `.expect()`, indicating a pattern to audit for production paths."
- source_event: "unwraps_and_silent_fallbacks_rustacean.md"
  path: "src/adapters/version_source/install_script.rs"
  loc: "line 54"
  note: "`status.code().unwrap_or(-1)` acts as a silent fallback that masks actual failure details."
- source_event: "unwraps_and_silent_fallbacks_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "line 152"
  note: "`code.unwrap_or(-1)` acts as a silent fallback."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/app/container.rs`
- `crates/mev-internal/src/domain/error.rs`
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/app/cli/git.rs`
- `crates/mev-internal/src/app/cli/gh.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `src/adapters/version_source/install_script.rs`

## Constraints

- Errors must preserve semantic meaning and attach context where the system meets the world.
- Silent fallbacks are prohibited. Any fallback must be explicit, opt-in, and surfaced as a failure or a clearly logged, reviewed decision.
- Production paths must safely propagate errors rather than panicking with `expect` or `unwrap`.

## Acceptance Criteria

- `Box<dyn std::error::Error>` is completely removed from adapter return types and replaced with specific, domain-aligned error types (e.g., `AppError` or sub-variants).
- `unwrap_or(-1)` usages for process exit codes are removed and replaced with explicit checking and error forwarding for process execution failures.
- Production domain logic no longer invokes `unwrap()` or `expect()`.
