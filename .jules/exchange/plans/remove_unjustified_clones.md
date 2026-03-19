---
label: "refacts"
---

## Goal

Eliminate unjustified reflexive `.clone()` usages by aligning ownership structures to utilize borrowed values (`&str`, `&Path`) where appropriate, reducing unnecessary allocations and keeping references tighter to their data owners.

## Current State

Unjustified use of `.clone()` on `PathBuf` and `String` occurs in the codebase, leading to unnecessary memory allocations.
- `src/adapters/ansible/executor.rs`: The `AnsibleAdapter` test `test_build_command_success` clones the `PathBuf` for `ansible_dir` into the adapter constructor simply to reuse the original path later in the test assertions.
- `src/app/commands/backup/mod.rs`: `value_to_string` and `format_string` explicitly clone strings out of `serde_yaml::Value::String` into owned `String` instances rather than leveraging references (`&str`) or `Cow` to avoid the initial allocation.

## Plan

1. Use `replace_with_git_merge_diff` to modify `src/adapters/ansible/executor.rs` to remove the unnecessary clone of `ansible_dir` in `test_build_command_success`. Ensure you extract `ansible_dir_path = dir.path().join("ansible")` before passing `ansible_dir` by value to `AnsibleAdapter`, and use `ansible_dir_path` in subsequent assertions.
2. Run `cargo test` to verify `src/adapters/ansible/executor.rs` changes compile and pass.
3. Use `replace_with_git_merge_diff` to modify `src/app/commands/backup/mod.rs` to import `std::borrow::Cow`.
4. Run `cargo check` to verify imports compile.
5. Use `replace_with_git_merge_diff` to modify `value_to_string` in `src/app/commands/backup/mod.rs` to return `Cow<'_, str>` instead of `String`. Map `serde_yaml::Value::String(s)` to `Cow::Borrowed(s.as_str())`, and map other variants to `Cow::Owned`.
6. Run `cargo check` to verify changes to `value_to_string` compile.
7. Use `replace_with_git_merge_diff` to modify `format_string` in `src/app/commands/backup/mod.rs` to use `Cow<'_, str>` for the internal `value` variable. Replace the `.clone()` of the default string with `Cow::Borrowed(s.as_str())`. Return the owned string at the end by calling `.into_owned()` if `serde_json::to_string` is unapplicable.
8. Run `cargo check` to verify changes to `format_string` compile.
9. Use `replace_with_git_merge_diff` to modify `format_value` in `src/app/commands/backup/mod.rs` to handle the `Cow` return type from `value_to_string`. In the `_` arm, capture `let value = if raw_value.is_empty() { value_to_string(&def.default) } else { Cow::Borrowed(raw_value) };` and adjust the final `serde_json::to_string` return to unwrap or fallback to `value.into_owned()`.
10. Run `cargo test` in the root workspace to verify all implementation changes compile and tests pass.
11. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
12. Submit a pull request referencing this plan to finalize the structural updates and debt removal.

## Acceptance Criteria

- `AnsibleAdapter` and test configurations no longer clone paths unnecessarily.
- String extractions from YAML sequences borrow `&str` instead of cloning `String`.

## Risks

- Changing `value_to_string` to return a `Cow` could cause lifetime compilation errors if the references to `serde_yaml::Value` do not outlive the scope where the `Cow` is consumed into an owned `String`.
- Removing the path `.clone()` in the test might lead to borrow checker errors if the original value is moved without properly creating and retaining a separate path reference for the assertions.
