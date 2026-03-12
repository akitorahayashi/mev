---
label: "docs"
created_at: "2024-05-24"
author_role: "consistency"
confidence: "high"
---

## Problem

`README.md` claims "Rust-first architecture with embedded Ansible assets" in its introductory paragraph, but lists `pipx` and `ansible (via pipx)` as prerequisites. While playbooks are embedded (as confirmed by `src/adapters/ansible/locator.rs`), the `ansible-playbook` binary itself is required to run via pipx (as confirmed by `src/adapters/ansible/executor.rs` expecting it at `venvs/ansible/bin/ansible-playbook`). The phrasing "embedded Ansible assets" incorrectly implies that Ansible itself is bundled, leading users to believe the prerequisite steps are outdated.

## Goal

Clarify in `README.md` that while Ansible *playbooks* are embedded as assets within the Rust binary, the `ansible-playbook` execution engine itself must still be installed on the host system via `pipx`.

## Context

Users reading "embedded Ansible assets" might skip the prerequisite installation of Ansible, leading to an execution failure when the CLI attempts to invoke `ansible-playbook` via pipx. The documentation must clearly distinguish between the embedded playbooks and the external execution engine dependency.

## Evidence

- path: "README.md"
  loc: "5"
  note: "Claims 'Rust-first architecture with embedded Ansible assets.' which is ambiguous regarding the execution engine."

- path: "README.md"
  loc: "26-34"
  note: "Lists `pipx` and `ansible` as prerequisites, which contradicts the potential assumption that 'embedded Ansible' means no external dependencies."

- path: "src/adapters/ansible/executor.rs"
  loc: "41-50"
  note: "The implementation explicitly resolves the `ansible-playbook` binary from the host's `pipx` environment, confirming the execution engine is not embedded."

## Change Scope

- `README.md`
