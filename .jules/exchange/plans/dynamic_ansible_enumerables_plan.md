---
label: "refacts"
---

## Goal

Generate tag groups, backup targets, and profiles dynamically from authoritative sources (the Ansible playbook and filesystem configuration structure) to eliminate hardcoded enumerable variants in the Rust domain layer and ensure single-source-of-truth ownership.

## Current State

- **Implementation**:
  - `src/domain/tag.rs`: Hardcodes `FULL_SETUP_TAGS` and `tag_groups()`. This creates a dual-path of truth with the Ansible playbook.
  - `src/domain/profile.rs`: Hardcodes the `Profile` enum (`Macbook`, `MacMini`, `Common`) and mappings. Fails to derive dynamically from `src/assets/ansible/roles/*/config/profiles/`.
  - `src/domain/backup_target.rs`: Hardcodes the `BackupTarget` enum (`System`, `Vscode`). Fails to discover targets based on configured roles.
  - `src/adapters/ansible/executor.rs`: Parses `playbook.yml` but fails to expose a rich enough catalog to construct profiles and targets dynamically.
- **Tests**:
  - `tests/cli/list.rs`: Assertions may rely on hardcoded output structure for profiles/targets.
  - Domain tests in `src/domain/tag.rs`, `src/domain/profile.rs`, and `src/domain/backup_target.rs` assert against static lists.
- **Documentation**:
  - `docs/architecture.md`: Mentions `macbook/mac-mini` as static examples rather than dynamically configured values.
  - `docs/usage.md`: Uses `macbook` and `vscode` as static examples. Doesn't clarify that these are derived from the Ansible configuration.

## Plan

1. **Ansible Adapter Enhancement (`src/adapters/ansible/executor.rs` & `src/domain/ports/ansible.rs`)**:
   - Establish the `AnsiblePort` as the authoritative source for enumerables.
   - Add catalog discovery methods: `tag_groups()`, `full_setup_tags()`, `profiles()`, `machine_profiles()`, and `backup_targets()`.
   - Implement dynamic file system scanning inside the adapter (e.g., scanning `roles/*/config/profiles/` for profiles, and identifying roles that support backups).

2. **Domain Layer Dereification (`src/domain/tag.rs`, `src/domain/profile.rs`, `src/domain/backup_target.rs`)**:
   - Eliminate hardcoded enums (`Profile`, `BackupTarget`) and static arrays (`FULL_SETUP_TAGS`).
   - Re-own the validation and resolution logic by having these domain modules request valid enumerables dynamically from the `AnsiblePort`.
   - Update `validate_machine_profile`, `validate_profile`, and `validate_backup_target` to leverage the dynamic catalog.

3. **Command Layer Integration (`src/app/commands/create/mod.rs`, `src/app/commands/backup/mod.rs`, `src/domain/execution_plan.rs`)**:
   - Update commands to request the valid tags, profiles, and backup targets dynamically via the injected `DependencyContainer` (specifically `ctx.ansible`).

4. **Domain and Adapter Tests**:
   - Refactor unit tests in `tag.rs`, `profile.rs`, and `backup_target.rs` to use mock `AnsiblePort` data instead of testing static enums.
   - Add parsing tests to `executor.rs` to guarantee dynamic profile and backup target discovery correctly navigates the test `tempdir` filesystem.

5. **CLI Integration Tests**:
   - Update `tests/cli/list.rs` to dynamically assert against the actual loaded profiles/targets if required, or relax strict string matching if output formatting changes.

6. **Documentation Updates (`docs/architecture.md`, `docs/usage.md`)**:
   - Update `docs/architecture.md` to explicitly state that profiles and backup targets are dynamically discovered from the Ansible asset structure.
   - Adjust `docs/usage.md` to clarify that arguments like `macbook` or `vscode` are dynamically sourced, ensuring examples remain valid.

## Acceptance Criteria

- `src/domain/tag.rs`, `src/domain/profile.rs`, and `src/domain/backup_target.rs` contain zero hardcoded enum variants or static arrays of tags/profiles.
- The `AnsiblePort` implementation dynamically discovers profiles and targets from the filesystem and playbook.
- Existing tests pass, demonstrating that observable behavior is preserved with the new dynamic architecture.
- Documentation accurately describes the dynamic nature of configuration enumerables.

## Risks

- Dynamic validation depends heavily on accurate filesystem structure. If the `src/assets/ansible/` structure deviates, validation may fail silently. (Mitigation: Add robust error reporting in the adapter).
- Removing hardcoded CLI aliases (like `mmn` for `mac-mini`) might break existing automation workflows unless dynamic alias resolution is implemented.
