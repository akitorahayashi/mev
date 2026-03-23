---
label: "refacts"
created_at: "2024-03-23"
author_role: "data_arch"
confidence: "high"
---

## Problem

Ansible execution tags (e.g., `python-platform`, `nodejs-tools`) are hardcoded in the Rust domain layer (`src/domain/tag.rs`) while the single source of truth for tag definitions is the Ansible playbook itself (`src/assets/ansible/playbook.yml`).

## Goal

Generate tag groups and lists dynamically from the authoritative source (the Ansible playbook) instead of hardcoding them in the Rust domain layer to eliminate maintenance burden and ensure extensibility.

## Context

The `AnsibleAdapter` already implements logic to parse the `playbook.yml` file and resolve the mapping of tags to roles. However, `src/domain/tag.rs` maintains an independent, hardcoded copy of full setup tags (`FULL_SETUP_TAGS`) and tag groups (`tag_groups()`). This violates the Single Source of Truth principle. If a tag is added or modified in the playbook, it must be manually updated in the Rust code to be available for the `create` or `make` commands. It is also error-prone and violates the design rule: "Never hardcode enumerable values. Generate them dynamically from authoritative sources (catalog, registry, schema) to ensure extensibility and eliminate maintenance burden."

## Evidence

- path: "src/domain/tag.rs"
  loc: "tag_groups"
  note: "Defines hardcoded `tag_groups()` that duplicate information natively present in or derived from the playbook."
- path: "src/domain/tag.rs"
  loc: "FULL_SETUP_TAGS"
  note: "Defines hardcoded `FULL_SETUP_TAGS` that duplicate information natively present in or derived from the playbook."
- path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter::new"
  note: "The `AnsibleAdapter::new` function uses logic to parse `playbook.yml` to extract roles and tags, showing that the playbook is already read and parsed at runtime."

## Change Scope

- `src/domain/tag.rs`
- `src/adapters/ansible/executor.rs`
- `src/app/commands/create/mod.rs`
