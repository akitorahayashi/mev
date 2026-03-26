---
label: "tests"
implementation_ready: false
---

## Goal

Provide rigorous test coverage ensuring `runtime_assets.rs` can safely extract embedded directories.

## Problem

The extraction logic in `src/adapters/ansible/runtime_assets.rs`, responsible for dumping embedded Ansible assets to disk, is significantly under-tested. Only 2 of 25 lines are covered, presenting a risk to critical provisioning execution.

## Evidence

- source_event: "uncovered_adapters_ansible_runtime_assets_cov.md"
  path: "src/adapters/ansible/runtime_assets.rs"
  loc: "2/25"
  note: "Primary extraction logic and failure modes are uncovered."

## Change Scope

- `src/adapters/ansible/runtime_assets.rs`
- `tests/ansible/`

## Constraints

- Add safe test boundaries and fakes for asset extraction.

## Acceptance Criteria

- `runtime_assets.rs` logic is fully verified by isolated tests.