---
label: "refacts"
created_at: "2026-03-11"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term "common" is used pervasively throughout the codebase as a directory name, file name, and profile identifier, violating the design rule against ambiguous names or responsibilities.

## Goal

Rename "common" to a specific, domain-aligned term that clearly identifies its responsibility, or restructure the configuration to eliminate the need for a generic fallback bucket.

## Context

The repository has an explicit design rule: "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers."
Currently, `common` is used in Ansible role structures to hold shared configurations and as a fallback profile identifier. This usage restates directory scope rather than identifying a single, specific responsibility, making the codebase harder to navigate and understand.

## Evidence

- path: "src/domain/profile.rs"
  loc: "12"
  note: "Defines `Profile::Common`, which is used as a generic profile fallback rather than a specific machine configuration."

- path: "src/domain/backup_target.rs"
  loc: "53"
  note: "Uses `common` as a hardcoded subpath for backup targets, hiding the actual responsibility."

- path: "src/assets/ansible/roles/*/config/common"
  loc: "directory structure"
  note: "Almost all Ansible roles have a `common` subdirectory for shared configurations, such as `src/assets/ansible/roles/ssh/config/common`, `src/assets/ansible/roles/llm/config/common`, etc."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/assets/ansible/roles/*/config/common`
