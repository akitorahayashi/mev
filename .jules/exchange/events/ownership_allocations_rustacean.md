---
label: "refacts"
created_at: "2024-05-23"
author_role: "rustacean"
confidence: "high"
---

## Problem

Scattered `.clone()` calls without a documented boundary rationale (lifetime, caching, threading, ergonomics) obscure the single source of truth and indicate potential architecture or ownership flaws.

## Goal

Highlight instances where ownership transfer or reference passing should replace reflexive cloning, minimizing unnecessary allocations.

## Context

Reflexive use of `.clone()` appease the borrow checker but often points to a lack of clear ownership semantics. They add overhead and can mask design problems that could be solved with better lifetimes or ownership transfer.

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter"
  note: "`self.tags_by_role.clone()` performs a deep clone of a HashMap without a stated rationale."

- path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Multiple `.clone()` calls on `embedded_dir` and `manifest_dir` without a clear boundary justification."

- path: "src/app/container.rs"
  loc: "DependencyContainer"
  note: "Clones `ansible_dir` and `local_config_root` when creating adapters, instead of passing references or taking ownership."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/adapters/ansible/locator.rs`
- `src/app/container.rs`