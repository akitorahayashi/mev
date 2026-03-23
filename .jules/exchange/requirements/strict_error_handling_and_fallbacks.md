---
label: "refacts"
implementation_ready: false
---

## Goal

Ensure all domain and boundary errors use explicit typed errors instead of `Box<dyn std::error::Error>`, and eliminate silent `.unwrap_or_default()` fallbacks in favor of explicit error propagation.

## Problem

Widespread use of `Box<dyn std::error::Error>` collapses typed error semantics, preventing downstream error handling. Furthermore, silent fallbacks using `.unwrap_or_default()` when parsing standard paths or configuration data hide critical failures and drift configurations.

## Evidence

- source_event: "silent_fallbacks_rustacean.md"
  path: "src/adapters/git/cli.rs"
  loc: "read_config"
  note: "`read_config` uses `.unwrap_or_default()` if `git config` fails, silently suppressing config fetch errors."
- source_event: "silent_fallbacks_rustacean.md"
  path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Locator logic defaults silently to returning an empty string/path when resolution fails."
- source_event: "untyped_errors_domain_rustacean.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "from_repo_arg, from_remote_url"
  note: "`from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`, erasing the distinction between parsing and IO failures."
- source_event: "untyped_errors_domain_rustacean.md"
  path: "src/app/container.rs"
  loc: "new, for_identity"
  note: "`Container::new` and `Container::for_identity` return dynamic errors instead of `AppError`."

## Change Scope

- `src/adapters/git/cli.rs`
- `src/adapters/ansible/locator.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `src/app/container.rs`

## Constraints

- Silent fallbacks are prohibited. Any fallback mechanism must be explicit, opt-in, and surfaced either as a failure or a clearly logged, reviewed decision.
- Typed errors must be used in the domain logic.

## Acceptance Criteria

- `read_config` and `locate_ansible_dir` do not use silent `unwrap_or_default()` and propagate errors explicitly.
- `from_repo_arg`, `from_remote_url`, and container initialization functions return typed errors rather than `Box<dyn Error>`.