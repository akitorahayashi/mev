---
label: "refacts"
created_at: "2025-03-14"
author_role: "rustacean"
confidence: "medium"
---

## Problem

Reflexive `.clone()` usage for string and path allocation across adapters and commands bypasses proper lifetime and ownership design, increasing allocation overhead without a justified threading or caching boundary.

## Goal

Align ownership structures by leveraging borrowed values (`&str`, `&Path`) for ephemeral operations or transferring ownership properly, rather than sprinkling `.clone()` to appease the borrow checker.

## Context

Cloning values without a documented boundary rationale (such as long-lived storage, threads, or complex ergonomics) violates ownership architecture. It obscures the single source of truth and adds unnecessary runtime allocations.

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "192"
  note: "The entire `tags_by_role` HashMap is cloned when returning it from `AnsiblePort::tags_by_role()`, which could be avoided if the trait returned an iterator or reference."
- path: "src/app/container.rs"
  loc: "45"
  note: "`ansible_dir.clone()` and `local_config_root.clone()` are passed to `AnsibleAdapter::new`, suggesting the container might be needlessly cloning paths instead of passing references or taking ownership."
- path: "src/adapters/identity_store/local_json.rs"
  loc: "81"
  note: "Clones the path `self.identity_path` just to return it from a getter, which could return a `&Path`."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/app/container.rs`
- `src/adapters/identity_store/local_json.rs`
- `src/domain/ports/ansible.rs`
