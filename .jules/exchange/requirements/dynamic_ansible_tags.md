---
label: "refacts"
implementation_ready: false
---

## Goal

Eliminate redundantly hardcoded Ansible tags by generating or resolving them dynamically from the authoritative `playbook.yml` catalog.

## Problem

Ansible tag groups are hardcoded within Rust (`src/domain/tag.rs`). This results in a dual source of truth, causing a maintenance burden and a risk of divergence between the playbook definition and the Rust runtime.

## Evidence

- source_event: "hardcoded_tags_data_arch.md"
  path: "src/domain/tag.rs"
  loc: "tag_groups, FULL_SETUP_TAGS"
  note: "`tag_groups` and `FULL_SETUP_TAGS` define static lists of tags that duplicate information present in the Ansible playbook."
- source_event: "hardcoded_tags_data_arch.md"
  path: "src/domain/execution_plan.rs"
  loc: "ExecutionPlan::full_setup"
  note: "`ExecutionPlan::full_setup` relies on the hardcoded `FULL_SETUP_TAGS`."

## Change Scope

- `src/domain/tag.rs`
- `src/domain/execution_plan.rs`

## Constraints

- Parsing of the `playbook.yml` catalog must be efficient or done safely at runtime.

## Acceptance Criteria

- Hardcoded tags and execution orders are removed from `src/domain/tag.rs`.
- The system reads tags dynamically from the playbook source.
