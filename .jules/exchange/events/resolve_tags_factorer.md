---
label: "refacts"
created_at: "2024-04-17"
author_role: "factorer"
confidence: "high"
---

## Problem

Signature Drift and Placement: The `resolve_tags` function in `src/domain/tag.rs` takes a tag string and a `tag_groups: &HashMap<String, Vec<String>>` to expand tags. However, the concept of resolving tags, including groups, is tightly coupled to the catalog data which is owned by `AnsiblePort`.

## Goal

Move `resolve_tags` to be a method on the `AnsiblePort` trait since `AnsiblePort` owns the `tag_groups` configuration.

## Context

The function signature expects callers to extract `tag_groups` from `AnsiblePort` and pass it in, which is an example of an incomplete/drifting signature where ambient state is passed manually instead of coupling behavior to the owner.

## Evidence

- path: "src/domain/tag.rs"
  loc: "8-10"
  note: "Definition of resolve_tags that requires passing the HashMap state."
- path: "src/app/commands/make/mod.rs"
  loc: "35"
  note: "Usage: tag::resolve_tags(tag_input, ctx.ansible.tag_groups())"

## Change Scope

- `src/domain/tag.rs`
- `src/domain/ports/ansible.rs`
- `src/app/commands/make/mod.rs`
