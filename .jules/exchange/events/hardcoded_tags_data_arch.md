---
label: "refacts"
created_at: "2024-05-24"
author_role: "data_arch"
confidence: "high"
---

## Problem

Ansible tags and groups (`tag_groups` and `FULL_SETUP_TAGS`) are hardcoded in `src/domain/tag.rs` rather than being generated dynamically from the authoritative Ansible playbook catalog.

## Goal

Eliminate redundant definitions by dynamically resolving tags and tag groups from the Ansible `playbook.yml` catalog.

## Context

Hardcoding enumerable values like tags creates a dual source of truth (the Rust codebase and the Ansible playbooks). This leads to a maintenance burden and the risk of divergence. The codebase should treat the Ansible catalog as the single source of truth for available tags and execution orders.

## Evidence

- path: "src/domain/tag.rs"
  loc: "tag_groups, FULL_SETUP_TAGS"
  note: "`tag_groups` and `FULL_SETUP_TAGS` define static lists of tags that duplicate information present in the Ansible playbook."
- path: "src/domain/execution_plan.rs"
  loc: "ExecutionPlan::full_setup"
  note: "`ExecutionPlan::full_setup` relies on the hardcoded `FULL_SETUP_TAGS`."

## Change Scope

- `src/domain/tag.rs`
- `src/domain/execution_plan.rs`
