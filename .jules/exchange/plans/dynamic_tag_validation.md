---
label: "refacts"
---

## Goal

Generate or validate available tags dynamically from authoritative Ansible catalog definitions instead of keeping a duplicated, manual array up to date. Eliminate technical debt and duplication.

## Current State

- `src/domain/tag.rs`: Hardcodes `tag_groups()` and `FULL_SETUP_TAGS`, defining static arrays of tags which duplicate the underlying Ansible playbook logic.
- `src/domain/execution_plan.rs`: Relies on `FULL_SETUP_TAGS` to create a plan for full environment setup.
- `src/app/commands/deploy_configs.rs`: Maps tags and fetches config directories via `role_for_tag`, skipping dependencies dynamically instead of explicitly validating the presence of the tags or failing effectively if tags are completely unknown.
- `src/app/commands/create/mod.rs`: Relies on `FULL_SETUP_TAGS` to build the full setup plan.
- `src/app/commands/make/mod.rs`: Relies on `tag::resolve_tags(tag_input)` to resolve shorthand group tags.

## Plan

1. Modify `src/domain/ports/ansible.rs` to expose ordered tags:
   - Add `fn ordered_tags(&self) -> Vec<String>;` to `AnsiblePort`.
   - Ensure the docstring states it retrieves all tags in the order defined by the catalog.

2. Modify `src/adapters/ansible/executor.rs` to parse tags in order:
   - Update `load_catalog` in `src/adapters/ansible/executor.rs` to return `(HashMap<String, Vec<String>>, HashMap<String, String>, Vec<String>)` (adding `ordered_tags`).
   - Iterate over the roles sequence and record every tag sequentially. Insert them into a new `Vec<String> ordered_tags`. Keep existing `tag_to_role` and `tags_by_role`.
   - Return all three values.
   - Update `AnsibleAdapter` struct to store `ordered_tags: Vec<String>`.
   - Update `AnsibleAdapter::new` to initialize it.
   - Implement `fn ordered_tags(&self) -> Vec<String> { self.ordered_tags.clone() }` for `AnsibleAdapter` in the `AnsiblePort` impl.

3. Update Fakes in `src/testing/ansible.rs`:
   - Add `pub ordered_tags: Vec<String>` to `FakeAnsiblePort`.
   - Initialize it in `FakeAnsiblePort::new`.
   - Implement `fn ordered_tags(&self) -> Vec<String> { self.ordered_tags.clone() }` for `FakeAnsiblePort`.

4. Modify `src/domain/execution_plan.rs` to take tags dynamically:
   - Remove `use crate::domain::tag::FULL_SETUP_TAGS;`
   - Change `pub fn full_setup(profile: Profile, verbose: bool) -> Self` to `pub fn full_setup(profile: Profile, tags: Vec<String>, verbose: bool) -> Self`.
   - Construct `Self { profile, tags, verbose }`.
   - Update `full_setup_contains_all_tags` test to pass a dummy vector of tags and assert against it.

5. Modify `src/app/commands/create/mod.rs` to use `ordered_tags`:
   - Remove `use crate::domain::tag::FULL_SETUP_TAGS;`
   - Delete the `invalid` validation block that compares `FULL_SETUP_TAGS` to `all_catalog_tags`. We are dynamically generating the tags from the catalog so this validation is obsolete.
   - Get the tags dynamically: `let ordered_tags = ctx.ansible.ordered_tags();`
   - Call `ExecutionPlan::full_setup(profile, ordered_tags, verbose);`

6. Modify `src/app/commands/make/mod.rs` to resolve groups natively:
   - Remove `use crate::domain::tag;` and `tag::resolve_tags`.
   - Replace `tag::resolve_tags` with native resolution against the adapter:
     ```rust
     let tags_to_run = if let Some(role_tags) = ctx.ansible.tags_by_role().get(tag_input) {
         role_tags.clone()
     } else {
         vec![tag_input.to_string()]
     };
     ```
   - Keep the existing `ctx.ansible.role_for_tag(t)` validation.

7. Modify `src/app/commands/list/mod.rs` to display dynamic groups:
   - Remove `use crate::domain::tag;` and the call to `tag::tag_groups()`.
   - Instead, print `Roles (can be used as tag groups):`.
   - Use `ctx.ansible.tags_by_role()` directly to print the "groups".
   - Sort the role names alphabetically and print them with their mapped tags.

8. Modify `src/app/commands/deploy_configs.rs` to strictly validate tags:
   - Instead of silently using `continue;` on `let Some(role) = ansible.role_for_tag(tag) else { continue; };`, return an explicit error:
     ```rust
     let Some(role) = ansible.role_for_tag(tag) else {
         return Err(AppError::InvalidTag(format!("tag '{tag}' does not map to any known role")));
     };
     ```

9. Delete `src/domain/tag.rs`:
   - Delete the file entirely using `rm src/domain/tag.rs`.
   - Remove `pub mod tag;` from `src/domain/mod.rs`.

10. Run `cargo check` to verify that the refactored traits, adapters, and deleted files compile correctly.

11. Run `cargo test` to ensure the dynamic tag generation is correct, no regressions were introduced, and tests still pass.

## Acceptance Criteria

- Tag definitions are dynamically derived or validated against Ansible roles, and static hardcoded arrays are removed.

## Risks

- Generating execution tags dynamically might not preserve execution order. We mitigate this by extracting `ordered_tags` exactly as they are defined in `playbook.yml`.
