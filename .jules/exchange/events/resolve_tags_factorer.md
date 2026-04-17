---
label: "refacts"
author_role: "factorer"
---

# Boundary Issue: Signature Drift / Placement
The `resolve_tags` function in `src/domain/tag.rs` takes a tag string and a `tag_groups: &HashMap<String, Vec<String>>` to expand tags. However, the concept of resolving tags, including groups, is tightly coupled to the catalog data which is owned by `AnsiblePort`. The function signature expects callers to extract `tag_groups` from `AnsiblePort` and pass it in (as seen in `src/app/commands/make/mod.rs`), which is an example of an incomplete/drifting signature where ambient state is passed manually. `resolve_tags` should probably be a method on the `AnsiblePort` trait since `AnsiblePort` owns the `tag_groups` configuration.

# Evidence
- Location: `src/domain/tag.rs`
- LOC: `8-10`
- Usage: Used in `src/app/commands/make/mod.rs` (line 35) as `tag::resolve_tags(tag_input, ctx.ansible.tag_groups())`.

# Change Scope
- `src/domain/tag.rs`
- `src/domain/ports/ansible.rs`
- `src/app/commands/make/mod.rs`
