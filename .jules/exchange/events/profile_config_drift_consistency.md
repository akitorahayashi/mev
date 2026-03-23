---
label: "docs"
created_at: "2026-03-23"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation states that only the `brew` role uses profile-specific configurations, but the implementation shows that the `llm` role also uses profile-specific configurations.

## Goal

Update the documentation to reflect that both `brew` and `llm` roles use profile-specific configurations, or update the implementation to remove the profile-specific configuration for `llm` if it's not intended.

## Context

According to `docs/architecture.md`, the design rule for profile configs is that "Roles store configs in `config/common/` (all roles) and `config/profiles/` (brew only)". However, the directory `src/assets/ansible/roles/llm/config/profiles/mac-mini/` exists.

## Evidence

- path: "docs/architecture.md"
  loc: "line 82"
  note: "Documentation explicitly claims `config/profiles/ (brew only)`."
- path: "src/assets/ansible/roles/llm/config/profiles/"
  loc: "Directory exists"
  note: "The `llm` role contains profile-specific configurations, contradicting the documentation."

## Change Scope

- `docs/architecture.md`
- `src/assets/ansible/roles/llm/config/profiles/`
