---
label: "refacts"
created_at: "2026-03-14"
author_role: "taxonomy"
confidence: "high"
---

## Problem
The term `common` is used extensively as a generic fallback or shared directory in Ansible role paths (e.g., `roles/*/config/common`), violating the design rule against using ambiguous names like "common" for scope/organization.

## Goal
Rename the generic `common` directory to a specific, domain-relevant term that describes its purpose (e.g., `global`, `shared`, `default`, or simply flattened into `config/` if no other variants exist).

## Context
Directories named `common` hide their actual responsibility and create a dumping ground for unrelated configurations. First principles dictate that names should encode the correct boundary or domain concept rather than restating package scope.

## Evidence
- path: "src/domain/backup_target.rs"
  loc: "pub fn subpath"
  note: "Hardcodes `common` as the subdirectory within the role config directory."
- path: "src/assets/ansible/roles/system/config/common/system.yml"
  loc: "com.apple.WindowManager"
  note: "Example of a `common` directory used to store system definitions."

## Change Scope
- `src/domain/backup_target.rs`
- `src/assets/ansible/roles/*/config/*`
