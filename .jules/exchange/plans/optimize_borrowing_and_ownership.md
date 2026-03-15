---
label: "refacts"
---

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`) where appropriate, reducing unnecessary allocations and keeping references tighter to their data owners.

## Current State

Currently, there is a proliferation of ownership transfers using `.clone()` returning from getters or in constructors where borrowing would be sufficient and correct:

- `src/domain/ports/ansible.rs`: The interface `AnsiblePort` defines methods `tags_by_role` returning a cloned `HashMap<String, Vec<String>>` and `role_for_tag` returning an `Option<String>`, forcing all implementations to clone data that can be borrowed.
- `src/domain/ports/identity_store.rs`: The interface `IdentityStore` defines `identity_path` as returning a `PathBuf`, forcing implementations to clone internal path states when returning.
- `src/adapters/ansible/executor.rs`: `AnsibleAdapter::new` takes owned `PathBuf` arguments, while `tags_by_role` clones its internal dictionary, and `role_for_tag` clones the internal role string.
- `src/app/container.rs`: Passes `ansible_dir.clone()` and `local_config_root.clone()` to `AnsibleAdapter::new` just to fulfill ownership constraints, where taking ownership of path clones is arbitrary for long-lived components initialized in the container.
- `src/adapters/identity_store/local_json.rs`: The implementation of `identity_path` uses `.clone()` on `PathBuf` instead of returning a borrowed `&Path`.
- `src/app/commands/list/mod.rs`: Consumes the fully cloned hash map from `tags_by_role`, whereas it could just iterate over borrowed items.
- `src/app/commands/make/mod.rs`: Uses the owned string returned from `role_for_tag` without needing it beyond checking presence.
- `src/app/commands/deploy_configs.rs`: Uses the owned string returned from `role_for_tag` where a borrowed slice would suffice.
- `src/app/commands/identity/mod.rs`: Uses the owned `PathBuf` from `identity_path`.

Tests and Documentation:
- Existing tests (e.g. `tests/runtime.rs`, `tests/security.rs`) cover CLI contracts but do not explicitly verify borrow behavior. Tests will inherently pass if the command outcomes remain identical, enforcing observable behavior preservation.
- No explicit documentation on component memory footprints exists, but the structural signatures will require updates to align with borrowing norms.

## Plan

1. Update `AnsiblePort` in `src/domain/ports/ansible.rs`:
   - Change `fn tags_by_role(&self) -> &HashMap<String, Vec<String>>;`
   - Change `fn role_for_tag(&self, tag: &str) -> Option<&str>;`
2. Update `IdentityStore` in `src/domain/ports/identity_store.rs`:
   - Change `fn identity_path(&self) -> &Path;`
3. Update `AnsibleAdapter` in `src/adapters/ansible/executor.rs`:
   - Change `fn tags_by_role(&self) -> &HashMap<String, Vec<String>>` to return `&self.tags_by_role`.
   - Change `fn role_for_tag(&self, tag: &str) -> Option<&str>` to return `self.tag_to_role.get(tag).map(|s| s.as_str())`.
   - Ensure `AnsibleAdapter` stores what it needs, and investigate if `DependencyContainer` in `src/app/container.rs` can pass `ansible_dir` without `.clone()` or just keep its copies but optimize `AnsibleAdapter` creation parameters. The specific fix will be to change `AnsibleAdapter::new` signatures if helpful, or eliminate redundant `.clone()` calls inside `container.rs` when initializing `AnsibleAdapter`. We will fix the clones in `container.rs` by potentially passing references or reusing owned data appropriately without `ansible_dir.clone()` and `local_config_root.clone()`.
4. Update `IdentityFileStore` in `src/adapters/identity_store/local_json.rs`:
   - Implement `identity_path(&self) -> &Path`.
5. Update dependent callers in command handlers:
   - `src/app/commands/list/mod.rs`: Iterate over references from `tags_by_role`.
   - `src/app/commands/deploy_configs.rs` and `src/app/commands/make/mod.rs`: Adjust `role_for_tag` handlers to work with `&str`.
   - `src/app/commands/identity/mod.rs`: Handle `&Path` returned by `identity_path()`.

## Acceptance Criteria

- `AnsiblePort` methods `tags_by_role` and `role_for_tag` return references (`&HashMap` and `Option<&str>`).
- `IdentityStore::identity_path` returns `&Path` instead of `PathBuf`.
- Unjustified `.clone()` calls on strings and paths identified in the requirement are eliminated.
- External behavior remains identical across all command invocations, confirmed by tests.

## Risks

- Call sites using modified `AnsiblePort` or `IdentityStore` getters might still attempt to consume the value as owned, leading to borrow-checker errors. Call sites must be carefully updated to properly borrow data.
- Refactoring `DependencyContainer`'s initialization paths to avoid `.clone()` might lead to ownership conflicts if paths are heavily shared across adapters. Ownership models within the container must be managed tightly.
