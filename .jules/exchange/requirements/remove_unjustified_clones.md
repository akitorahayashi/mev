---
label: "refacts"
implementation_ready: false
---

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`) where appropriate, reducing unnecessary allocations and keeping references tighter to their data owners.

## Problem

Unjustified use of `.clone()` on `PathBuf` occurs when passing directories within `AnsibleAdapter`. The adapter copies the paths via `clone()` instead of utilizing references (`&Path`) when traversing or returning values, leading to unnecessary allocations. Additionally, in `src/app/commands/backup/mod.rs`, strings are explicitly cloned when processing YAML sequences. Cloning strings or paths simply to appease the borrow checker or to avoid lifetime annotations when the data outlives the required scope goes against ownership principles.

## Evidence

- source_event: "unjustified_clone_usage_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "419"
  note: "`ansible_dir: ansible_dir.clone(),` is an unnecessary clone as `AnsibleAdapter` could take ownership or the test can be restructured to avoid cloning the tempdir path."
- source_event: "unjustified_clone_usage_rustacean.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "152, 216"
  note: "`serde_yaml::Value::String(s) => s.clone()` unnecessary cloning of string values out of the YAML tree."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/app/commands/backup/mod.rs`

## Constraints

- Use references where possible.

## Acceptance Criteria

- `AnsibleAdapter` and test configurations no longer clone paths unnecessarily.
- String extractions from YAML sequences borrow `&str` instead of cloning `String`.
