---
label: "refacts"
created_at: "2026-03-12"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term "base" is used in an ambiguous context for paths/URLs instead of precise domain-specific terms.

## Goal

Replace occurrences of "base" with more descriptive terms indicating their specific role, such as "root".

## Context

Using terms like "base path" or "release base URL" lacks specificity and can lead to confusion about what exactly the variable represents. "Base" is restricted by `AGENTS.md`.

## Evidence

- path: "src/adapters/identity_store/paths.rs"
  loc: "line 3"
  note: "Uses 'base path' instead of a more specific term like 'configuration root'."
- path: "src/assets/ansible/roles/rust/config/common/tools.yml"
  loc: "line 6"
  note: "Uses 'release base URL' instead of 'release root URL'."

## Change Scope

- `src/adapters/identity_store/paths.rs`
- `src/assets/ansible/roles/rust/config/common/tools.yml`
