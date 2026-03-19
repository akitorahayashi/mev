---
label: "refacts"
---

## Goal

Refactor ambiguous taxonomy terms (`common`, `helpers`) into precise, domain-aligned language (`default`, `integration commands`) to comply with core design principles and improve comprehensibility. Eliminate all technical debt tied to these terms across the domain layer, CLI layer, configuration directories, and playbooks.

## Current State

- `src/domain/profile.rs`: Defines `Profile::Common`, canonical string mapping (`"common"`), aliases, and validation logic (`validate_machine_profile_rejects_common`) involving the ambiguous term `common`.
- `src/domain/backup_target.rs`: Uses `"common"` as a hardcoded subdirectory path (`subpath`) for Ansible role configs.
- `src/app/cli/make.rs`: Uses `"common"` in CLI parameter defaults and help text for the `profile` argument.
- `src/app/cli/mod.rs`: Employs "helpers" in the descriptions for `Git` and `Gh` internal subcommands (`"Git helpers."` and `"GitHub CLI helpers."`).
- `crates/mev-internal/src/app/cli/mod.rs`: Employs "helpers" in the descriptions for `Git` and `Gh` internal subcommands (`"Git helpers."` and `"GitHub CLI helpers."`).
- `src/app/commands/backup/mod.rs`: Contains the internal documentation comment `// Shared helpers` which violates the ambiguous names rule.
- `src/assets/ansible/roles/**/config/common`: Configuration fallback directories use the folder name `common` across all Ansible roles.
- `src/assets/ansible/roles/**/tasks/*.yml`: Multiple Ansible playbooks use the hardcoded path component `common` to reference configuration files (e.g., `{{ local_config_root }}/.../common/...`).
- `src/assets/ansible/roles/shell/config/common/alias/nodejs/coder.sh`: Uses hardcoded path component `common`.
- `src/assets/ansible/roles/nodejs/config/common/coder/AGENTS.md`: Contains the rule text that includes the word `common` and `helpers`.
- `src/assets/ansible/roles/editor/tasks/xcode.yml`: Uses `role_path + '/config/common/xcode/*.yml'`.
- `src/assets/ansible/roles/rust/AGENTS.md`: Uses `config/common/tools.yml` and `config/common/platforms.yml`.

## Plan

1. Use `run_in_bash_session` to rename all directories named `common` within `src/assets/ansible/roles/` to `default` (e.g., `find src/assets/ansible/roles -name "common" -type d -execdir mv {} default \;`). Followed by an explicit check using `find` to confirm successful renaming.
2. Use `replace_with_git_merge_diff` to modify `src/domain/profile.rs`:
   - Rename the variant `Profile::Common` to `Profile::Default`.
   - Update `as_str` mapping to `"default"`.
   - Update `aliases` mapping to `["def"]`.
   - Update `PROFILE_ALIASES` to map `("default", Profile::Default)` and `("def", Profile::Default)`.
   - Update function doc comment `Validate any profile including common` to `Validate any profile including default`.
   - Update test `resolves_canonical_profiles` to assert `"default"`.
   - Update test `resolves_aliases` to assert `"def"`.
   - Update test `validate_machine_profile_rejects_common` to `validate_machine_profile_rejects_default` and assert `"default"`.
   - Update test `profile_as_str_roundtrips` to assert `"default"`.
3. Use `replace_with_git_merge_diff` to modify `src/domain/backup_target.rs`:
   - Update `subpath` method to return `"default"`.
4. Use `replace_with_git_merge_diff` to modify `src/app/cli/make.rs`:
   - Update help text: `Profile to use (default, macbook/mbk, mac-mini/mmn).`
   - Update macro default: `default_value = "default"`.
5. Use `replace_with_git_merge_diff` to modify `src/app/cli/mod.rs` to replace `"Git helpers."` with `"Git integration commands."` and `"GitHub CLI helpers."` with `"GitHub CLI integration commands."`.
6. Use `replace_with_git_merge_diff` to modify `crates/mev-internal/src/app/cli/mod.rs` to replace `"Git helpers."` with `"Git integration commands."` and `"GitHub CLI helpers."` with `"GitHub CLI integration commands."`.
7. Use `replace_with_git_merge_diff` to modify `src/app/commands/backup/mod.rs` to replace the comment `// Shared helpers` with `// Shared definitions`.
8. Use `run_in_bash_session` to perform a search and replace for the string `common` with `default` in all Ansible `.yml`, `.sh`, and `.md` files under `src/assets/ansible/roles/` using `find` and `sed` (e.g., `find src/assets/ansible/roles -type f \( -name "*.yml" -o -name "*.sh" -o -name "*.md" \) -exec sed -i '' 's/common/default/g' {} +`). Include verification with `rg "common" src/assets/ansible/roles/`.
9. Use `run_in_bash_session` with `rg "common"` and `rg "helpers"` globally to ensure no other occurrences exist in source code that need manual cleanup.
10. Run tests `cargo test` in the root and `cd crates/mev-internal && cargo test` to verify changes.

## Acceptance Criteria

- The term `common` is eradicated from `Profile` definitions, domain logic, command parameters, test cases, and configuration folder layouts.
- The term `helpers` is removed from all CLI descriptions and internal documentation comments.
- Tests assert correct resolution of `default` instead of `common`.
- The CLI command `make` functions with `default` correctly without parsing errors.

## Risks

- Playbooks might fail if they dynamically build paths relying on `common` that are missed by text search. Mitigated by comprehensive regex find/replace and testing.
- Hardcoded references in bash aliases or shell files might break. Mitigated by searching for `/common` and updating.
