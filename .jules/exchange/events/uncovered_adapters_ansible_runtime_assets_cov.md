---
label: "tests"
created_at: "2024-05-24"
author_role: "cov"
confidence: "high"
---

## Problem

The extraction logic in `src/adapters/ansible/runtime_assets.rs`, responsible for dumping embedded Ansible assets to disk, is significantly under-tested.

## Goal

Provide rigorous test coverage ensuring that `runtime_assets.rs` can extract embedded directories securely to temporary spaces, controlling risks around corrupt or missing files that could halt provisioning.

## Context

Tarpaulin shows only 2 out of 25 lines covered for `runtime_assets.rs`. This file is in the critical path for any ansible execution. If asset extraction fails or creates bad permissions, the entire tool chain will fail in production.

## Evidence

- path: "src/adapters/ansible/runtime_assets.rs"
  loc: "2/25"
  note: "Primary extraction logic and failure modes (e.g., IO errors, missing assets) are uncovered."
- path: "src/adapters/ansible/locator.rs"
  loc: "16/34"
  note: "Resolution logic bridging embedded extraction and local overrides is only partially tested."

## Change Scope

- `src/adapters/ansible/runtime_assets.rs`
- `tests/ansible/`
