---
label: "refacts"
created_at: "2026-04-17"
author_role: "modeler"
confidence: "medium"
---

## Problem

Tag concepts (tag name, list of tags) are universally represented as primitive `String` and `Vec<String>` across the domain and port interfaces, lacking a distinct domain type.

## Goal

Consider introducing a `Tag` and `TagGroup` struct or alias to clarify the domain model, encapsulate validation logic, and provide type safety against arbitrary strings being passed as tags.

## Context

In `src/domain/tag.rs`, `src/domain/execution_plan.rs`, and `src/domain/ports/ansible.rs`, tags are represented as `String`. This is primitive obsession. A specific domain model for tags would centralize validation and distinguish raw user input from known, validated catalog tags.

## Evidence

- path: "src/domain/execution_plan.rs"
  loc: "8"
  note: "`ExecutionPlan` stores tags as `Vec<String>`"
- path: "src/domain/ports/ansible.rs"
  loc: "23-26"
  note: "`AnsiblePort` methods like `all_tags`, `tag_groups`, and `full_setup_tags` return raw `String` collections."
- path: "src/domain/tag.rs"
  loc: "9"
  note: "`resolve_tags` accepts and returns primitive `String` types."

## Change Scope

- `src/domain/tag.rs`
- `src/domain/execution_plan.rs`
- `src/domain/ports/ansible.rs`
