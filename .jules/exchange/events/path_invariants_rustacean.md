---
label: "refacts"
created_at: "2024-05-24"
author_role: "rustacean"
confidence: "high"
---

## Problem
The `ResolvedAnsibleDir` invariant is discarded immediately in `DependencyContainer::new`, converting to a raw `PathBuf` and losing the guarantee of structural validity for the Ansible directory.

## Goal
Preserve the `ResolvedAnsibleDir` type through the system, particularly into `AnsibleAdapter`, to ensure that invalid paths cannot be represented or passed around.

## Context
`ResolvedAnsibleDir` provides proof that the directory exists and contains necessary components (or cleans up a temp dir). However, `DependencyContainer::new` destructs it into a `PathBuf` (`into_parts()`) and hands it to `AnsibleAdapter::new(PathBuf, ...)`. This forces the adapter to re-verify the directory state or assume it's valid, violating the principle of making invalid states unrepresentable.

## Evidence
- path: "src/app/container.rs"
  loc: "43"
  note: "The ResolvedAnsibleDir is destructed into its parts."
- path: "src/app/container.rs"
  loc: "45"
  note: "A raw PathBuf is cloned and passed to AnsibleAdapter::new."
- path: "src/adapters/ansible/executor.rs"
  loc: "67"
  note: "AnsibleAdapter::new accepts a raw PathBuf."

## Change Scope
- `src/app/container.rs`
- `src/adapters/ansible/executor.rs`