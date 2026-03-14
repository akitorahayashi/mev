---
label: "refacts"
implementation_ready: false
---

## Goal
Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`) where appropriate.

## Problem
Several adapters and command layers use `.clone()` arbitrarily on strings and paths without a valid threading or caching justification. This practice bypasses proper ownership architecture, obscures the single source of truth, and incurs unnecessary allocation overheads.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "ownership_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "192"
  note: "The entire `tags_by_role` HashMap is cloned when returning it from `AnsiblePort::tags_by_role()`."
- source_event: "ownership_rustacean.md"
  path: "src/app/container.rs"
  loc: "45"
  note: "`ansible_dir.clone()` and `local_config_root.clone()` are passed to `AnsibleAdapter::new`."
- source_event: "ownership_rustacean.md"
  path: "src/adapters/identity_store/local_json.rs"
  loc: "81"
  note: "Clones the path `self.identity_path` just to return it from a getter."

## Change Scope
- `src/adapters/ansible/executor.rs`
- `src/domain/ports/ansible.rs`
- `src/app/container.rs`
- `src/adapters/identity_store/local_json.rs`

## Constraints
- Lifetimes or references must be used to eliminate deep cloning of data structures on hot paths unless required for thread boundary dispatch.

## Acceptance Criteria
- `AnsiblePort` methods return references instead of cloned structures where possible.
- Getters for paths return `&Path` instead of `PathBuf`.
