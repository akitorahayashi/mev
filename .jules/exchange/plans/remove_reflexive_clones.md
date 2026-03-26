---
label: "refacts"
---

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`, `Cow`) where appropriate.

## Current State

The codebase contains several instances where variables are cloned unnecessarily to satisfy the borrow checker when taking ownership or references would suffice, leading to unnecessary allocations.

- `src/domain/execution_plan.rs`: In the `make_contains_provided_tags` test, `tags` is cloned to pass into `ExecutionPlan::make`, but could be borrowed or moved.
- `src/adapters/ansible/locator.rs`: In tests, `manifest_dir` and `embedded_dir` are cloned unnecessarily inside closures when passing values.
- `src/adapters/ansible/executor.rs`: Multiple instances of `.clone()` and `.cloned()` exist, especially in tests. Functions like `resolve_ansible_playbook_bin_with_env` accept a closure returning `Option<OsString>`, forcing callers to use `.cloned()` on `HashMap` values instead of returning an `Option<&OsStr>`.

## Plan

1. Modify `src/domain/execution_plan.rs` to fix tests:
   - In the `make_contains_provided_tags` test, remove `tags.clone()` when calling `ExecutionPlan::make`. Move `tags` into `ExecutionPlan::make` instead of cloning it. Define a temporary slice or reference array to use for the final assertion if needed, or inline the value creation.

2. Modify `src/adapters/ansible/locator.rs` to fix `locate_ansible_dir_with` and tests:
   - Update `locate_ansible_dir_with`'s `manifest_dir` parameter type to accept `Option<&Path>` instead of `Option<PathBuf>`, avoiding clones at call sites. Adjust the function implementation to use `Path` references.
   - Update the call site in `locate_ansible_dir` to pass `manifest_dir.as_deref()`.
   - In `prefers_manifest_assets_over_embedded_cache` and `uses_embedded_assets_when_manifest_assets_are_missing` tests, remove `.clone()` from `manifest_dir` and `embedded_dir` by passing them by reference or moving them into the closure.

3. Modify `src/adapters/ansible/executor.rs` to remove `.clone()` and `.cloned()`:
   - Update `load_catalog` to avoid `s.clone()` and `name.clone()` where possible by borrowing or restructuring the code. E.g., change `serde_yaml::Value::String(s) => vec![s.clone()]` to take ownership or iterate correctly.
   - Update `resolve_ansible_playbook_bin_with_env` to accept a closure `F: Fn(&str) -> Option<T> where T: AsRef<std::ffi::OsStr>`. Change the closure call sites to return `Option<&OsStr>` without calling `.cloned()`.
   - Update `test_resolve_ansible_playbook_bin_*` tests. Change `env_map.get(k).cloned()` to just `env_map.get(k)`. Change `bin_path.clone().into()` to `bin_path.into()` by moving it into the map.

4. Run all tests to verify behavior preservation:
   - Run `cargo test` in the workspace.
   - Run `cd crates/mev-internal && cargo test`.

5. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. Submit the change.

## Acceptance Criteria

- Unjustified `.clone()` usages in the specified files are removed.
- Tests pass successfully without regression.
- Function signatures correctly utilize borrowed types (`&Path`, `&OsStr`) where ownership is not required.

## Risks

- Changing function signatures to use borrows (`&Path`, `AsRef<OsStr>`) could cause compilation errors if not applied uniformly across all call sites.
