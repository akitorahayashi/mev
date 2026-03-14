---
label: "refacts"
implementation_ready: false
---

## Goal
Decouple application command orchestration from concrete filesystem and Ansible adapter internals by using port abstractions (`FsPort`, `AnsiblePort`), enabling unit testing with in-process test doubles.

## Problem
Application commands like `deploy_configs` and `config` directly use `std::fs` and construct Ansible role paths internally (`ansible_dir.join(...)`), bypassing port boundaries. They also depend on a concrete `DependencyContainer` which exposes internal paths publicly (`pub ansible_dir`), making orchestration logic untestable in isolation without slow, brittle integration tests.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "adapter_fs_abstraction_structural_arch.md"
  path: "src/app/commands/deploy_configs.rs"
  loc: "42"
  note: "Uses `std::fs::remove_dir_all(&target)` instead of abstracting through the `FsPort`."
- source_event: "adapter_fs_abstraction_structural_arch.md"
  path: "src/app/commands/config/mod.rs"
  loc: "63-71"
  note: "`copy_dir_recursive` uses `std::fs::create_dir_all`, `std::fs::read_dir`, and `std::fs::copy` instead of port abstractions."
- source_event: "ansible_port_dependency_structural_arch.md"
  path: "src/app/commands/config/mod.rs"
  loc: "34"
  note: "Directly constructs paths like `ctx.ansible_dir.join(\"roles\").join(role_name).join(\"config\")`."
- source_event: "ansible_port_dependency_structural_arch.md"
  path: "src/app/container.rs"
  loc: "24"
  note: "The `DependencyContainer` struct leaves `pub ansible_dir: PathBuf` exposed."
- source_event: "application_orchestration_test_doubles_qa.md"
  path: "src/app/commands/create/mod.rs"
  loc: "14"
  note: "`pub fn execute(ctx: &DependencyContainer, ...)` tightly couples logic to concrete adapters."
- source_event: "application_orchestration_test_doubles_qa.md"
  path: "tests/cli/backup.rs"
  loc: "12-14"
  note: "`backup_alias_bk_is_accepted` explicitly expects failure due to missing ansible assets."

## Change Scope
- `src/app/commands/`
- `src/domain/ports/fs.rs`
- `src/domain/ports/ansible.rs`
- `src/app/container.rs`
- `src/testing/`
- `tests/cli/`

## Constraints
- Command layers must accept interfaces (e.g., `&dyn AnsiblePort`) instead of concrete structs.
- No `std::fs` calls allowed directly inside `src/app/commands/`.

## Acceptance Criteria
- `deploy_configs` and `config` commands use `FsPort` and `AnsiblePort` exclusively.
- `DependencyContainer` encapsulates `ansible_dir`.
- Application commands have unit tests using fast test doubles in `src/testing/`.
