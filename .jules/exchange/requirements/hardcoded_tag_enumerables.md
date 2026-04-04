---
label: "refacts"
implementation_ready: false
---

## Goal

Generate enumerable values for tags dynamically from an authoritative source (e.g., the Ansible catalog/registry) to avoid hardcoding.

## Problem

Tags and tag groups are hardcoded in `src/domain/tag.rs` rather than being generated dynamically from an authoritative source (like the Ansible catalog).

## Context

Design principles state "Enumerable values are generated dynamically from authoritative sources (catalog, registry, schema) rather than hardcoded." Currently, `FULL_SETUP_TAGS` and `tag_groups()` are hardcoded arrays/maps. This requires manual synchronization whenever the Ansible roles change.

## Evidence

- path: "src/domain/tag.rs"
  loc: "6-16"
  note: "`tag_groups()` and `FULL_SETUP_TAGS` are hardcoded in the domain layer, lacking an authoritative source generation."

## Change Scope

- `src/domain/tag.rs`
- `src/app/commands/create/mod.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
