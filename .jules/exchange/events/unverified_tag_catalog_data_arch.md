---
label: "refacts"
created_at: "2024-05-23"
author_role: "data_arch"
confidence: "medium"
---

## Problem

Tag values, used to drive Ansible execution mapping, are hardcoded in the domain model rather than explicitly validated against a single source of truth or the external Ansible asset catalog itself.

## Goal

Generate or validate available tags dynamically from authoritative Ansible catalog definitions instead of keeping a duplicated, manual array up to date.

## Context

The Single Source of Truth principle states that each concept must have one canonical representation. Hardcoded tag groups (`FULL_SETUP_TAGS`, `tag_groups()`) duplicate knowledge of the underlying Ansible roles, leading to a disconnect if roles are added or removed dynamically.

## Evidence

- path: "src/domain/tag.rs"
  loc: "5-33"
  note: "`tag_groups()` and `FULL_SETUP_TAGS` define static arrays containing hardcoded string tags."
- path: "src/app/commands/deploy_configs.rs"
  loc: "21"
  note: "`deploy_for_tags` consumes the tags blindly and maps them via `role_for_tag`, silently skipping missing dependencies without explicit validation."

## Change Scope

- `src/domain/tag.rs`
- `src/domain/execution_plan.rs`
