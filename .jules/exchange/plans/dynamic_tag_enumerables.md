---
label: "refacts"
---

## Goal

Generate enumerable values for tags dynamically from an authoritative source (Ansible catalog) to avoid hardcoding tags and tag groups in the codebase.

## Current State

- `src/domain/tag.rs`: Hardcodes `tag_groups()` and `FULL_SETUP_TAGS` instead of resolving them dynamically from the Ansible catalog.
- `src/app/commands/create/mod.rs`: Reads `FULL_SETUP_TAGS` from the hardcoded `src/domain/tag.rs`.
- `src/domain/ports/ansible.rs`: Defines the `AnsiblePort` trait, which needs functions to dynamically query tag groups and full setup tags.
- `src/adapters/ansible/executor.rs`: Parses the Ansible playbook `playbook.yml` to extract role-to-tag mappings but currently does not expose the `tag_groups` or `full_setup_tags` variables declared inside it.
- `src/domain/execution_plan.rs`: Relies on `FULL_SETUP_TAGS` directly from `src/domain/tag.rs` when initializing `full_setup(profile, verbose)`.
- `src/assets/ansible/playbook.yml`: Currently missing explicit declarations of `tag_groups` and `full_setup_tags` in the playbook vars.

## Plan

1. Elevate the `playbook.yml` as the single authoritative source for both role-to-tag mappings and tag groupings. Introduce a `vars` block to explicitly define `tag_groups` and `full_setup_tags` within the playbook asset.
2. Expand the `AnsiblePort` interface in `src/domain/ports/ansible.rs` to expose `tag_groups` and `full_setup_tags` queries, shifting ownership of these enumerables from hardcoded domain rules to the port-driven adapter.
3. Update the `AnsibleAdapter` in `src/adapters/ansible/executor.rs` to parse the `vars` block during playbook loading, maintaining these new lists in memory, and fulfilling the expanded `AnsiblePort` contract.
4. Eliminate structural debt in `src/domain/tag.rs` by removing the hardcoded `FULL_SETUP_TAGS` constant and `tag_groups()` function, altering `resolve_tags` to receive tag groupings via dependency injection.
5. Refactor the `ExecutionPlan` domain boundary and application command orchestration (`create` and `make` commands) to dynamically query the `AnsiblePort` for tag enumerables instead of reading static references, preserving the existing observable behavior while enforcing the new dynamic ownership model.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
