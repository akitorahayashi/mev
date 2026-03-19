---
label: "refacts"
created_at: "2024-05-23"
author_role: "rustacean"
confidence: "high"
---

## Problem

Unjustified use of `.clone()` on `PathBuf` when passing directories within `AnsibleAdapter`. The adapter copies the paths via `clone()` instead of utilizing references (`&Path`) when traversing or returning values, leading to unnecessary allocations. Also in `src/app/commands/backup/mod.rs` strings are explicitly cloned when processing YAML sequences.

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`) where appropriate, reducing unnecessary allocations and keeping references tighter to their data owners.

## Context

Clone/RC must pay rent: copies and shared ownership exist only with a named boundary rationale. Cloning strings or paths simply to appease the borrow checker or to avoid lifetime annotations when the data outlives the required scope goes against ownership principles.

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "419"
  note: "`ansible_dir: ansible_dir.clone(),` is an unnecessary clone as `AnsibleAdapter` could take ownership or the test can be restructured to avoid cloning the tempdir path."
- path: "src/app/commands/backup/mod.rs"
  loc: "152, 216"
  note: "`serde_yaml::Value::String(s) => s.clone()` unnecessary cloning of string values out of the YAML tree."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/app/commands/backup/mod.rs`
