---
label: "docs"
implementation_ready: false
---

## Goal

Clarify in `README.md` that while Ansible playbooks are embedded, the external `ansible-playbook` binary must be installed via `pipx`.

## Problem

The `README.md` states "Rust-first architecture with embedded Ansible assets," which implies that the Ansible execution engine is bundled. However, it requires `pipx` and `ansible (via pipx)` to run, which creates confusion for users installing the prerequisites.

## Evidence

- source_event: "ansible_prerequisite_contradiction_consistency.md"
  path: "README.md"
  loc: "5"
  note: "Claims 'Rust-first architecture with embedded Ansible assets.' which is ambiguous regarding the execution engine."
- source_event: "ansible_prerequisite_contradiction_consistency.md"
  path: "README.md"
  loc: "26-34"
  note: "Lists `pipx` and `ansible` as prerequisites, which contradicts the potential assumption that 'embedded Ansible' means no external dependencies."
- source_event: "ansible_prerequisite_contradiction_consistency.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "41-50"
  note: "The implementation explicitly resolves the `ansible-playbook` binary from the host's `pipx` environment, confirming the execution engine is not embedded."

## Change Scope

- `README.md`

## Constraints

- The README update should be concise and clearly distinguish between embedded assets and external execution dependencies.

## Acceptance Criteria

- `README.md` clearly states that the `ansible-playbook` binary must be installed on the host system via `pipx`.
