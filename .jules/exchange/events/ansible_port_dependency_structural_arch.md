---
label: "refacts"
created_at: "2026-03-14"
author_role: "structural_arch"
confidence: "high"
---

## Problem

The Application command orchestration (`src/app/commands/deploy_configs.rs` and `src/app/commands/config/mod.rs`) depend directly on Ansible internals or the filesystem layout of the Ansible directory rather than abstracting this via Ports or pure data mapping.

## Goal

Ensure core application logic interacts with abstract Ports, removing direct references to Ansible file layouts (e.g. `roles/{role}/config`) inside command execution paths. The boundary between "Ansible as an implementation detail" and the pure application logic should be strictly enforced.

## Context

Commands like `config` and `deploy_configs` are manually reconstructing Ansible role config paths instead of asking the `AnsiblePort` or a dedicated configuration adapter for the resolved paths. This breaks boundary sovereignty by bleeding the Ansible internal folder structure into the command layer. Furthermore, `DependencyContainer::new` exposes `ansible_dir` publicly which encourages other components to bypass adapters.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "34"
  note: "Directly constructs paths like `ctx.ansible_dir.join(\"roles\").join(role_name).join(\"config\")`, leaking adapter structure into command layer."

- path: "src/app/commands/deploy_configs.rs"
  loc: "47"
  note: "Directly accesses `ansible_dir.join(\"roles\").join(&role).join(\"config\")`, bypassing any port-level abstraction."

- path: "src/app/container.rs"
  loc: "24"
  note: "The `DependencyContainer` struct leaves `pub ansible_dir: PathBuf` exposed, enabling unchecked adapter-internal access."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/app/container.rs`
- `src/domain/ports/ansible.rs`