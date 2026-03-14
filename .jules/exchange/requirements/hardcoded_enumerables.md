---
label: "refacts"
implementation_ready: false
---

## Goal

Generate enumerable values dynamically from authoritative sources (like Ansible role definitions, catalogs, or registries) to ensure extensibility and eliminate maintenance burden.

## Problem

Enumerable values like Ansible tags and profiles are hardcoded in the domain layer rather than being dynamically generated from authoritative sources.

## Context

According to the design rules, enumerable values must be generated dynamically from authoritative sources rather than hardcoded. In the codebase, tags required for full setup and profiles are explicitly listed as static slices in the domain models, creating a maintenance burden whenever a new tag or profile is added in the configuration or Ansible roles.

## Evidence

- source_event: "hardcoded_enumerables_data_arch.md"
  path: "src/domain/tag.rs"
  loc: "FULL_SETUP_TAGS"
  note: "Hardcodes the list of tags for a full environment setup."
- source_event: "hardcoded_enumerables_data_arch.md"
  path: "src/domain/tag.rs"
  loc: "tag_groups"
  note: "Hardcodes the mappings of tag groups to specific tags."
- source_event: "hardcoded_enumerables_data_arch.md"
  path: "src/domain/profile.rs"
  loc: "all_profiles"
  note: "Hardcodes the available profiles instead of discovering them dynamically."

## Change Scope

- `src/domain/tag.rs`
- `src/domain/profile.rs`
- `src/adapters/ansible/locator.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
